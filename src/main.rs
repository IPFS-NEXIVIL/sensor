// extern crate i2cdev;
// mod kxtj3_accelerometer;
// use i2cdev::core::*;

use std::{sync::Arc, time::Duration};

// use cpal::traits::StreamTrait;
use rand::{rngs::StdRng, Rng, SeedableRng};
use tokio::signal;

mod mic;

#[tokio::main]
async fn main() {
    // let st = mic::mic();
    // st.play().expect("unable to play stream");
    let client = Arc::new(reqwest::blocking::Client::new());

    tokio::spawn({
        println!("STARTED");
        let mut rng = {
            let rng = rand::thread_rng();
            StdRng::from_rng(rng).unwrap()
        };
        let _client = client.clone();

        async move {
            loop {
                let _ = client
                    .clone()
                    .post("http://10.42.0.1:3000")
                    .body(format!("{}", rng.gen::<f64>().to_string()))
                    .send();
                tokio::time::sleep(Duration::from_millis(700)).await;
            }
        }
    });
    tokio::task::yield_now().await;

    loop {
        tokio::select! {
        _ = signal::ctrl_c() => {
            println!("SHUTDOWN");
            // cancel_token.notify_one();
            break
        }}
    }
}
