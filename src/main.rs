use chrono::{Local, Timelike};
use rodio::source::{SineWave, Source};
use rodio::{OutputStream, Sink};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink0 = Sink::try_new(&stream_handle).unwrap(); // 毎秒用
    let sink1 = Sink::try_new(&stream_handle).unwrap(); // 定時用

    let source0 = SineWave::new(1975.0)
        .take_duration(Duration::from_secs_f32(0.1))
        .amplify(0.20);
    let source1 = SineWave::new(440.0)
        .take_duration(Duration::from_secs_f32(0.25))
        .amplify(0.20);
    let source2 = SineWave::new(880.0)
        .take_duration(Duration::from_secs_f32(3.0))
        .amplify(0.20);

    loop {
        let now_time = Local::now();
        let now_seconds = now_time.second();
        let now_ms = now_time.nanosecond() / 1_000_000;

        if now_ms == 0 {
            if [0, 10, 20, 30, 40, 50].contains(&now_seconds) {
                sink1.append(source2.clone());
            } else if (57..60).contains(&now_seconds) {
                sink0.append(source1.clone());
            } else {
                sink0.append(source0.clone());
            }
            sleep(Duration::from_millis(100));
            println!("now_seconds: {}", now_seconds);
        }
    }
}
