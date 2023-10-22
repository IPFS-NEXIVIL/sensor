use std::sync::Arc;
use std::time::Duration;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{InputCallbackInfo, Sample, SampleFormat, Stream, StreamError};

pub fn mic() -> Stream {
    const MAX_AMPLITUDE_F32: f32 = (u16::MAX / 2) as f32; // i16 max value
    const ZERO_AMPLITUDE: u16 = 0;
    const MIN_DB: f32 = -96.0;
    // const TERMINAL_WIDTH: f32 = 0.8f32;
    // const FPS: u64 = 24;
    const DRAW_SLEEP_TIME: Duration = Duration::from_millis(700);
    fn db_fs(data: &[f32]) -> f32 {
        let max = data
            .iter()
            .map(|f| (f.to_owned() as i16).unsigned_abs() as u16)
            .max()
            .unwrap_or(ZERO_AMPLITUDE);

        (20.0f32 * (max as f32 / MAX_AMPLITUDE_F32).log10()).clamp(MIN_DB, 0.0)
    }

    let client = Arc::new(reqwest::blocking::Client::new());

    fn error(e: StreamError) {
        panic!("Error in input stream {:?}", e);
    }

    let host = cpal::default_host();

    let device = host
        .default_input_device()
        .expect("unable to get default device");

    let config = device
        .default_input_config()
        .expect("unable to get default input config");

    match config.sample_format() {
        SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            move |data: &[f32], info: &InputCallbackInfo| {
                let db = db_fs(data);
                // let time = info.timestamp().capture;
                // let _ = client
                //     .clone()
                //     .post("http://10.42.0.1:3000")
                //     .body(format!("{}", db))
                //     .send();
                println!("{}", db);
                std::thread::sleep(DRAW_SLEEP_TIME);
            },
            error,
            None,
        ),
        _ => panic!("bad format"),
    }
    .expect("unable to build stream")
}
