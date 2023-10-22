// extern crate i2cdev;
// mod kxtj3_accelerometer;
// use i2cdev::core::*;

use cpal::traits::StreamTrait;

mod mic;

// #[tokio::main]
fn main() {
    let st = mic::mic();
    st.play().expect("unable to play stream");
}
