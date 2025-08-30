use anyhow::{Context, Result};
use r2r::{self, sensor_msgs::msg::Image as RosImage};
use std::time::{Duration, Instant};

use opencv::{
    core as cv,
    prelude::*,
    videoio::{
        self, VideoCapture, VideoWriter,
        CAP_ANY, CAP_FFMPEG, CAP_V4L2,
        CAP_PROP_FRAME_WIDTH, CAP_PROP_FRAME_HEIGHT, CAP_PROP_FPS,
        CAP_PROP_FOURCC, CAP_PROP_CONVERT_RGB,
    },
};

fn now_builtin_time() -> r2r::builtin_interfaces::msg::Time {
    use std::time::{SystemTime, UNIX_EPOCH};
    let dur = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
    r2r::builtin_interfaces::msg::Time { sec: dur.as_secs() as i32, nanosec: dur.subsec_nanos() }
}

// 打开相机（纯 OpenCV），优先尝试 MJPG -> BGR
fn open_camera(dev: i32, w: i32, h: i32, req_fps: f64) -> Result<VideoCapture> {
    use anyhow::bail;

    // 依次尝试 OpenCV 的后端：V4L2 -> FFmpeg -> Any
    let backends = [CAP_V4L2, CAP_FFMPEG, CAP_ANY];
    let mut last_err: Option<anyhow::Error> = None;

    for backend in backends {
        // 尝试打开
        let mut cap = match VideoCapture::new(dev, backend) {
            Ok(c) => c,
            Err(e) => { last_err = Some(e.into()); continue; }
        };
        match cap.is_opened() {
            Ok(true) => {
                let _ = cap.set(CAP_PROP_FRAME_WIDTH,  w as f64);
                let _ = cap.set(CAP_PROP_FRAME_HEIGHT, h as f64);
                if req_fps > 0.0 { let _ = cap.set(CAP_PROP_FPS, req_fps); }
                if let Ok(fcc) = VideoWriter::fourcc('M','J','P','G') {
                    let _ = cap.set(CAP_PROP_FOURCC, fcc as f64);
                }
                let _ = cap.set(CAP_PROP_CONVERT_RGB, 1.0);
                return Ok(cap);
            }
            _ => {
                last_err = Some(anyhow::anyhow!(format!("backend {} not opened", backend)));
            }
        }
    }

    if let Some(e) = last_err { Err(e) } else { bail!("no backend opened") }
}

// 实测 FPS（抓帧计数/时间差），用于校准设备实际能跑的上限
fn calibrate_fps(cap: &mut VideoCapture, warmup: usize, seconds: f32) -> Result<f32> {
    // 预热（让曝光/缓冲收敛）
    for _ in 0..warmup { let _ = cap.grab()?; }

    let mut frame = cv::Mat::default();
    let start = Instant::now();
    let mut cnt: u32 = 0;
    while start.elapsed().as_secs_f32() < seconds {
        if !cap.read(&mut frame)? || frame.empty() { continue; }
        cnt += 1;
    }
    let secs = start.elapsed().as_secs_f32().max(0.001);
    Ok(cnt as f32 / secs)
}

fn main() -> Result<()> {
    // 环境变量：CAM_DEV/CAM_W/CAM_H/CAM_FPS（CAM_FPS=期望值；<=0 则不限制上限，只按实测）
    let dev: i32 = std::env::var("CAM_DEV").ok().and_then(|s| s.parse().ok()).unwrap_or(0);
    let w: i32   = std::env::var("CAM_W").ok().and_then(|s| s.parse().ok()).unwrap_or(640);
    let h: i32   = std::env::var("CAM_H").ok().and_then(|s| s.parse().ok()).unwrap_or(480);
    let req_fps: f64 = std::env::var("CAM_FPS").ok().and_then(|s| s.parse().ok()).unwrap_or(30.0);

    // ---- 打开相机（OpenCV）----
    let mut cap = open_camera(dev, w, h, req_fps)?;

    // 读回协商结果（可能与请求不同）
    let rw = cap.get(videoio::CAP_PROP_FRAME_WIDTH)?  as i32;
    let rh = cap.get(videoio::CAP_PROP_FRAME_HEIGHT)? as i32;
    let rfps_readback = cap.get(videoio::CAP_PROP_FPS)?; 
    eprintln!("[cam_cap] negotiated: {}x{}  CAP_PROP_FPS={:.2}", rw, rh, rfps_readback);

    let measured = calibrate_fps(&mut cap, 15, 2.0)?; 
    let mut max_fps = measured as f64;
    if rfps_readback > 0.5 {
        max_fps = max_fps.min(rfps_readback);
    }
    if req_fps > 0.5 {
        max_fps = max_fps.min(req_fps);
    }
    let limit_fps = (max_fps - 0.5).max(1.0);
    let frame_dt = Duration::from_secs_f64(1.0 / limit_fps);
    eprintln!(
        "[cam_cap] measured_fps={:.2}, limit_fps={:.2}  (req={:.2}, readback={:.2})",
        measured, limit_fps, req_fps, rfps_readback
    );

    // ---- ROS2 init ----
    let ctx = r2r::Context::create()?;
    let mut node = r2r::Node::create(ctx, "cam_cap", "")?;
    let pub_img = node.create_publisher::<RosImage>("/camera/image_raw", r2r::QosProfile::sensor_data())?;

    // 复用 Mat，避免每帧重新分配
    let mut frame = cv::Mat::default();

    // 帧率节拍器
    let mut last_pub = Instant::now();

    loop {
        // 读取一帧（BGR）
        if !cap.read(&mut frame)? || frame.empty() {
            // 读不到帧时，也别忙等，稍微歇一下
            std::thread::sleep(Duration::from_millis(2));
            continue;
        }
        if !frame.is_continuous() {
            frame = frame.clone();
        }

        let cols = frame.cols();
        let rows = frame.rows();
        let bytes = frame.data_bytes().context("get Mat bytes failed")?;

        // 组装 ROS Image（BGR8）
        let mut msg = RosImage::default();
        msg.header.frame_id = "camera".to_string();
        msg.header.stamp = now_builtin_time();
        msg.width = cols as u32;
        msg.height = rows as u32;
        msg.encoding = "bgr8".to_string();
        msg.step = (cols * 3) as u32;
        msg.data = bytes.to_vec(); // r2r 需要拥有一份缓冲

        pub_img.publish(&msg)?;
        node.spin_once(Duration::from_millis(1));

        // 按 limit_fps 节拍：若本帧发布过快，则 sleep 补足
        let elapsed = last_pub.elapsed();
        if elapsed < frame_dt {
            std::thread::sleep(frame_dt - elapsed);
        }
        last_pub = Instant::now();
    }
}
