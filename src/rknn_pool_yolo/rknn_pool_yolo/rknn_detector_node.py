#!/usr/bin/env python3
import cv2, time
from collections import deque

import rclpy
from sensor_msgs.msg import Image
from vision_msgs.msg import Detection2DArray, Detection2D, ObjectHypothesisWithPose
from cv_bridge import CvBridge
from rclpy.clock import Clock, ClockType
# 按你的脚本导入（和 rknnpool.py、func.py 同目录）
from .rknnpool import rknnPoolExecutor
from .func import det_func


SCORE_TH   = 0.10
MIN_BOX    = 16

def main():
    # 1) 只为“能发消息”而初始化 ROS
    rclpy.init()
    node = rclpy.create_node('rknn_detector_script')
    bridge = CvBridge()
    pub_det = node.create_publisher(Detection2DArray, '/rknn/detections', 10)

    pool = rknnPoolExecutor(rknnModel='/home/cat/yolov8s_i8.rknn', TPEs=9, func=det_func)

    # 3) 摄像头
    cap = cv2.VideoCapture(0, cv2.CAP_V4L2)
    if not cap.isOpened():
        raise RuntimeError(f'无法打开摄像头 /dev/video{DEVICE_ID}')

    w = int(cap.get(cv2.CAP_PROP_FRAME_WIDTH))
    h = int(cap.get(cv2.CAP_PROP_FRAME_HEIGHT))
    fps = int(cap.get(cv2.CAP_PROP_FPS)) or 30

    # 4) 预热
    for _ in range(10):
        ok, fr = cap.read()
        if not ok: break
        pool.put(fr,0)

    t_prev = time.time()
    fps_win = deque(maxlen=30)

    try:
        while True:
            ok, frame = cap.read()

            
            ts = Clock(clo0ck_type=ClockType.SYSTEM_TIME).now().to_msg()

            if not ok:
                break
            
            ts_ns = int(time.time_ns())
            rgb = cv2.cvtColor(frame_bgr, cv2.COLOR_BGR2RGB)
            stab_rgb = gf.process_rgb(rgb, ts_ns)          # HxWx3 RGB
            stab_bgr = cv2.cvtColor(stab_rgb, cv2.COLOR_RGB2BGR)

            pool.put(frame,ts)

            res, got , ts1= pool.get()
            if not got:
                continue
            
            dets, fr = res
            H, W = fr.shape[:2]

            msg = Detection2DArray()
            msg.header.stamp = ts
            msg.header.frame_id = "camera"

            for (x1, y1, x2, y2), sc, cid in (dets or []):
                if sc < SCORE_TH: 
                    continue
                if (x2 - x1) < MIN_BOX or (y2 - y1) < MIN_BOX: 
                    continue

                x1 = max(0, min(int(x1), W-1))
                y1 = max(0, min(int(y1), H-1))
                x2 = max(0, min(int(x2), W-1))
                y2 = max(0, min(int(y2), H-1))
                if x2 <= x1 or y2 <= y1: 
                    continue

                det = Detection2D()

                # 基本识别结果
                hyp = ObjectHypothesisWithPose()
                hyp.hypothesis.class_id = str(int(cid))
                hyp.hypothesis.score    = float(sc)
                det.results.append(hyp)

                # 只填 2D 框
                cx = 0.5 * (x1 + x2)
                cy = 0.5 * (y1 + y2)
                det.bbox.center.position.x = float(cx)
                det.bbox.center.position.y = float(cy)
                det.bbox.size_x   = float(x2 - x1)
                det.bbox.size_y   = float(y2 - y1)

                msg.detections.append(det)


                cv2.rectangle(fr, (x1, y1), (x2, y2), (0, 255, 0), 2)
                cv2.putText(fr, f'{int(cid)}:{sc:.2f}', (x1, max(0, y1 - 6)),
                            cv2.FONT_HERSHEY_SIMPLEX, 0.5, (0, 0, 0), 1)

            # FPS
            now = time.time(); dt = now - t_prev; t_prev = now
            if dt > 0:
                fps_win.append(1.0/dt)
            if fps_win:
                cv2.putText(fr, f'FPS: {sum(fps_win)/len(fps_win):.1f}',
                            (10,30), cv2.FONT_HERSHEY_SIMPLEX, 0.8, (0,0,0), 2)

            pub_det.publish(msg)

            # 窗口
            cv2.imshow('RKNN (no tracker)', fr)

            # ESC 退出
            if (cv2.waitKey(1) & 0xFF) == 27:
                break

    except KeyboardInterrupt:
        pass
    finally:
        cap.release()
        pool.release()
        cv2.destroyAllWindows()
        node.destroy_node()
        if rclpy.ok():
            rclpy.shutdown()

if __name__ == '__main__':
    main()
