use chrono::{Local, Timelike};
use rodio::buffer::SamplesBuffer;
use rodio::source::{SineWave, Source};
use rodio::{OutputStream, Sink};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink0 = Sink::try_new(&stream_handle).unwrap(); // 毎秒用
    let sink1 = Sink::try_new(&stream_handle).unwrap(); // 定時用

    loop {
        let now_time = Local::now();
        let now_seconds = now_time.second();
        let now_ms = now_time.nanosecond() / 1_000_000;

        if now_ms == 0 {
            if [0, 10, 20, 30, 40, 50].contains(&now_seconds) {
                sink1.append(create_source2());
            } else if (57..60).contains(&now_seconds) {
                sink0.append(create_source1());
            } else {
                sink0.append(create_source0());
            }
            sleep(Duration::from_millis(900));
            println!("{}", now_time.format("%H:%M:%S"));
        }
    }
}

fn create_source0() -> impl Source<Item = f32> {
    SineWave::new(1975.0)
        .take_duration(Duration::from_secs_f32(0.1))
        .amplify(0.20)
}

fn create_source1() -> impl Source<Item = f32> {
    SineWave::new(440.0)
        .take_duration(Duration::from_secs_f32(0.2))
        .amplify(0.20)
}

fn create_source2() -> SamplesBuffer<f32> {
    let sinwave = SineWave::new(880.0)
        .take_duration(Duration::from_secs_f32(3.5))
        .amplify(0.20)
        .enumerate()
        .map(move |(i, sample)| {
            let decay_start_sample = (1.0 * 48000.0) as usize; // 1 second * sample rate
            if i < decay_start_sample {
                sample // No decay for the first 1 second
            } else {
                let decay_factor = (-0.00005 * (i - decay_start_sample) as f32).exp();
                sample * decay_factor
            }
        })
        .collect::<Vec<_>>();

    SamplesBuffer::new(1, 48000, sinwave)
}
