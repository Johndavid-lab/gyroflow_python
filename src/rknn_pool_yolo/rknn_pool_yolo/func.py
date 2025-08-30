import cv2
import numpy as np
from rknnlite.api import RKNNLite
from types import SimpleNamespace
import time 

from .rknnpool import rknnPoolExecutor
PERSON_CLASS_ID = 0
IMG_SIZE   = 640
NUM_CLASS  = 80
CONF_TH    = 0.4
IOU_TH     = 0.45
DFL_DIM    = 16
STRIDES    = {80: 8, 40: 16, 20: 32}  # h,w里的h对应80/40/20时的stride
OBJ_THRESH, NMS_THRESH = 0.45, 0.45

def filter_boxes(boxes, box_confidences, box_class_probs):
    """Filter boxes with object threshold."""
    box_confidences = box_confidences.reshape(-1)
    candidate, class_num = box_class_probs.shape
 
    class_max_score = np.max(box_class_probs, axis=-1)
    classes = np.argmax(box_class_probs, axis=-1)
 
    _class_pos = np.where(class_max_score * box_confidences >= OBJ_THRESH)
    scores = (class_max_score * box_confidences)[_class_pos]
 
    boxes = boxes[_class_pos]
    classes = classes[_class_pos]
 
    return boxes, classes, scores
 
def nms_boxes(boxes, scores):
    """Suppress non-maximal boxes."""
    x = boxes[:, 0]
    y = boxes[:, 1]
    w = boxes[:, 2] - boxes[:, 0]
    h = boxes[:, 3] - boxes[:, 1]
 
    areas = w * h
    order = scores.argsort()[::-1]
 
    keep = []
    while order.size > 0:
        i = order[0]
        keep.append(i)
 
        xx1 = np.maximum(x[i], x[order[1:]])
        yy1 = np.maximum(y[i], y[order[1:]])
        xx2 = np.minimum(x[i] + w[i], x[order[1:]] + w[order[1:]])
        yy2 = np.minimum(y[i] + h[i], y[order[1:]] + h[order[1:]])
 
        w1 = np.maximum(0.0, xx2 - xx1 + 0.00001)
        h1 = np.maximum(0.0, yy2 - yy1 + 0.00001)
        inter = w1 * h1
 
        ovr = inter / (areas[i] + areas[order[1:]] - inter)
        inds = np.where(ovr <= NMS_THRESH)[0]
        order = order[inds + 1]
    return np.array(keep)
 
def dfl(position):
    """Distribution Focal Loss (DFL)."""
    n, c, h, w = position.shape
    p_num = 4
    mc = c // p_num
    y = position.reshape(n, p_num, mc, h, w)
    
    # Vectorized softmax
    e_y = np.exp(y - np.max(y, axis=2, keepdims=True))
    y = e_y / np.sum(e_y, axis=2, keepdims=True)
    
    acc_metrix = np.arange(mc).reshape(1, 1, mc, 1, 1)
    y = (y * acc_metrix).sum(2)
    return y
 
def box_process(position):
    """Process box coordinates."""
    grid_h, grid_w = position.shape[2:4]
    col, row = np.meshgrid(np.arange(0, grid_w), np.arange(0, grid_h))
    col = col.reshape(1, 1, grid_h, grid_w)
    row = row.reshape(1, 1, grid_h, grid_w)
    grid = np.concatenate((col, row), axis=1)
    stride = np.array([IMG_SIZE//grid_h, IMG_SIZE//grid_w]).reshape(1,2,1,1)
 
    position = dfl(position)
    box_xy  = grid + 0.5 - position[:,0:2,:,:]
    box_xy2 = grid + 0.5 + position[:,2:4,:,:]
    xyxy = np.concatenate((box_xy*stride, box_xy2*stride), axis=1)
 
    return xyxy
 
def yolov8_post_process(input_data):
    """YOLOv8 post-processing."""
    boxes, scores, classes_conf = [], [], []
    defualt_branch = 3
    pair_per_branch = len(input_data) // defualt_branch
    
    for i in range(defualt_branch):
        boxes.append(box_process(input_data[pair_per_branch*i]))
        classes_conf.append(input_data[pair_per_branch*i+1])
        scores.append(np.ones_like(input_data[pair_per_branch*i+1][:,:1,:,:], dtype=np.float32))
 
    def sp_flatten(_in):
        ch = _in.shape[1]
        _in = _in.transpose(0,2,3,1)
        return _in.reshape(-1, ch)
 
    boxes = [sp_flatten(_v) for _v in boxes]
    classes_conf = [sp_flatten(_v) for _v in classes_conf]
    scores = [sp_flatten(_v) for _v in scores]
 
    boxes = np.concatenate(boxes)
    classes_conf = np.concatenate(classes_conf)
    scores = np.concatenate(scores)
 
    boxes, classes, scores = filter_boxes(boxes, scores, classes_conf)
 
    nboxes, nclasses, nscores = [], [], []
    for c in set(classes):
        inds = np.where(classes == c)
        b = boxes[inds]
        c = classes[inds]
        s = scores[inds]
        keep = nms_boxes(b, s)
 
        if len(keep) != 0:
            nboxes.append(b[keep])
            nclasses.append(c[keep])
            nscores.append(s[keep])
 
    if not nclasses and not nscores:
        return None, None, None
 
    return np.concatenate(nboxes), np.concatenate(nclasses), np.concatenate(nscores)

# --- YOLO worker: (fid, frame_bgr) -> (fid, dets, frame_bgr) ---
def det_func(rknn_det, frame_bgr):
    H0, W0 = frame_bgr.shape[:2]
    # 走 NN 要 NHWC uint8；保持你原先的 640x640 resize（或 letterbox）
    inp = cv2.resize(frame_bgr, (IMG_SIZE, IMG_SIZE))
    inp = inp[:, :, ::-1]            # BGR->RGB
    inp = inp[None, ...].astype(np.uint8)  # [1,H,W,3] NHWC uint8
    out = rknn_det.inference(inputs=[inp], data_format=['nhwc'])
    
    boxes, classes, scores = yolov8_post_process(out)

    dets = []
    if boxes is not None and boxes.shape[0] > 0:
        sx, sy = W0 / IMG_SIZE, H0 / IMG_SIZE
        for (x1, y1, x2, y2), sc, cid in zip(boxes, scores, classes):
            # 只保留 person
            if int(cid) != PERSON_CLASS_ID:
                continue
            # 映射回原图坐标
            x1 = int(round(x1 * sx)); y1 = int(round(y1 * sy))
            x2 = int(round(x2 * sx)); y2 = int(round(y2 * sy))
            # 传 ByteTrack：([x1,y1,x2,y2], score, cls)；cls 固定 0
            dets.append(([x1, y1, x2, y2], float(sc), 0))

    # 返回原图用于后续画框/裁剪
    return dets, frame_bgr