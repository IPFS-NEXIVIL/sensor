// extern crate i2cdev;
// mod kxtj3_accelerometer;
// use i2cdev::core::*;

use tokio::signal;
use cpal::traits::StreamTrait;

mod mic;

#[tokio::main]
async fn main() {
    let st = mic::mic();
    st.play().expect("unable to play stream");
    loop {
        tokio::select! {
        _ = signal::ctrl_c() => {
            println!("SHUTDOWN");
            // cancel_token.notify_one();
            break
        }}
    }
}
