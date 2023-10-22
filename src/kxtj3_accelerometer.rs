use i2cdev::core::I2CDevice;
use std::error::Error;

#[derive(Debug)]
pub struct AccelerometerSample {
    /// x-axis G's
    pub x: f32,
    /// y-axis G's
    pub y: f32,
    /// z-axis G's
    pub z: f32,
}

/// Trait for sensors that provide access to accelerometer readings (3-axis)
pub trait Accelerometer {
    type Error: Error;

    /// Grab an accelerometer sample from the device
    fn accelerometer_sample(&mut self) -> Result<AccelerometerSample, Self::Error>;
}

const KXTJ3_DEVID: u8 = 0x35; //chip id
const KXTJ3_RANGE: f32 = (2 * 16384) as f32;

const KXTJ3_XOUT_HPF_L: u8 = 0x00; /* 0000 0000 */
const KXTJ3_XOUT_HPF_H: u8 = 0x01; /* 0000 0001 */
const KXTJ3_YOUT_HPF_L: u8 = 0x02; /* 0000 0010 */
const KXTJ3_YOUT_HPF_H: u8 = 0x03; /* 0000 0011 */
const KXTJ3_ZOUT_HPF_L: u8 = 0x04; /* 0001 0100 */
const KXTJ3_ZOUT_HPF_H: u8 = 0x05; /* 0001 0101 */
const KXTJ3_XOUT_L: u8 = 0x06; /* 0000 0110 */
const KXTJ3_XOUT_H: u8 = 0x07; /* 0000 0111 */
const KXTJ3_YOUT_L: u8 = 0x08; /* 0000 1000 */
const KXTJ3_YOUT_H: u8 = 0x09; /* 0000 1001 */
const KXTJ3_ZOUT_L: u8 = 0x0A; /* 0001 1010 */
const KXTJ3_ZOUT_H: u8 = 0x0B; /* 0001 1011 */
const KXTJ3_ST_RESP: u8 = 0x0C; /* 0000 1100 */
const KXTJ3_WHO_AM_I: u8 = 0x0F; /* 0000 1111 */
const KXTJ3_TILT_POS_CUR: u8 = 0x10; /* 0001 0000 */
const KXTJ3_TILT_POS_PRE: u8 = 0x11; /* 0001 0001 */
const KXTJ3_INT_SRC_REG1: u8 = 0x15; /* 0001 0101 */
const KXTJ3_INT_SRC_REG2: u8 = 0x16; /* 0001 0110 */
const KXTJ3_STATUS_REG: u8 = 0x18; /* 0001 1000 */
const KXTJ3_INT_REL: u8 = 0x1A; /* 0001 1010 */
const KXTJ3_CTRL_REG1: u8 = 0x1B; /* 0001 1011 */
const KXTJ3_CTRL_REG2: u8 = 0x1C; /* 0001 1100 */
const KXTJ3_CTRL_REG3: u8 = 0x1D; /* 0001 1101 */
const KXTJ3_INT_CTRL_REG1: u8 = 0x1E; /* 0001 1110 */
const KXTJ3_INT_CTRL_REG2: u8 = 0x1F; /* 0001 1111 */
const KXTJ3_INT_CTRL_REG3: u8 = 0x20; /* 0010 0000 */
const KXTJ3_DATA_CTRL_REG: u8 = 0x21; /* 0010 0001 */
const KXTJ3_TILT_TIMER: u8 = 0x28; /* 0010 1000 */
const KXTJ3_WUF_TIMER: u8 = 0x29; /* 0010 1001 */
const KXTJ3_TDT_TIMER: u8 = 0x2B; /* 0010 1011 */
const KXTJ3_TDT_H_THRESH: u8 = 0x2C; /* 0010 1100 */
const KXTJ3_TDT_L_THRESH: u8 = 0x2D; /* 0010 1101 */
const KXTJ3_TDT_TAP_TIMER: u8 = 0x2E; /* 0010 1110 */
const KXTJ3_TDT_TOTAL_TIMER: u8 = 0x2F; /* 0010 1111 */
const KXTJ3_TDT_LATENCY_TIMER: u8 = 0x30; /* 0011 0000 */
const KXTJ3_TDT_WINDOW_TIMER: u8 = 0x31; /* 0011 0001 */
const KXTJ3_WUF_THRESH: u8 = 0x5A; /* 0101 1010 */
const KXTJ3_TILT_ANGLE: u8 = 0x5C; /* 0101 1100 */
const KXTJ3_HYST_SET: u8 = 0x5F; /* 0101 1111 */

/* CONTROL REGISTER 1 BITS */
const KXTJ3_DISABLE: u8 = 0x7F;
const KXTJ3_ENABLE: u8 = 1 << 7;
const KXTJ3_INT_ENABLE: u8 = 1 << 5;
/* INPUT_ABS CONSTANTS */
const FUZZ: u8 = 3;
const FLAT: u8 = 3;
/* RESUME STATE INDICES */
const RES_DATA_CTRL: u8 = 0;
const RES_CTRL_REG1: u8 = 1;
const RES_INT_CTRL1: u8 = 2;
const RESUME_ENTRIES: u8 = 3;

/* CTRL_REG1: set resolution, g-range, data ready enable */
/* Output resolution: 8-bit valid or 12-bit valid */
const KXTJ3_RES_8BIT: u8 = 0;
const KXTJ3_RES_12BIT: u8 = 1 << 6;
/* Output g-range: +/-2g, 4g, or 8g */
const KXTJ3_G_2G: u8 = 0;
const KXTJ3_G_4G: u8 = 1 << 3;
const KXTJ3_G_8G: u8 = 1 << 4;

/* DATA_CTRL_REG: controls the output data rate of the part */
#[repr(u8)]
enum KXTJ3DataRate {
    KXTJ3_ODR12_5F = 0,
    KXTJ3_ODR25F = 1,
    KXTJ3_ODR50F = 2,
    KXTJ3_ODR100F = 3,
    KXTJ3_ODR200F = 4,
    KXTJ3_ODR400F = 5,
    KXTJ3_ODR800F = 6,
}

/* kxtj3 */
const KXTJ3_PRECISION: u8 = 12;
const KXTJ3_BOUNDARY: f32 = (0x1 << (KXTJ3_PRECISION - 1)) as f32;
const KXTJ3_GRAVITY_STEP: f32 = KXTJ3_RANGE / KXTJ3_BOUNDARY;

pub struct KXTJ3Accelerometer<T: I2CDevice + Sized> {
    i2cdev: T,
}
impl<T> KXTJ3Accelerometer<T>
where
    T: I2CDevice + Sized,
{
    pub fn new(mut i2cdev: T) -> Result<KXTJ3Accelerometer<T>, T::Error> {
        // setup standy mode to configure
        
        // configure some defaults
        // set datarate
        i2cdev.smbus_write_byte_data(KXTJ3_DATA_CTRL_REG, KXTJ3DataRate::KXTJ3_ODR400F as u8)?;
        // set
        i2cdev.smbus_write_byte_data(KXTJ3_CTRL_REG1, KXTJ3_RES_12BIT | KXTJ3_G_2G)?;
        // i2cdev.smbus_write_byte_data(REGISTER_OFSX, 0xFD)?;
        // i2cdev.smbus_write_byte_data(REGISTER_OFSY, 0x03)?;
        // i2cdev.smbus_write_byte_data(REGISTER_OFSZ, 0xFE)?;
        
        i2cdev.smbus_write_byte_data(KXTJ3_CTRL_REG1, KXTJ3_ENABLE)?;
        // put device in measurement mode
        // i2cdev.smbus_write_byte_data(REGISTER_POWER_CTL, 0x08)?;

        Ok(KXTJ3Accelerometer { i2cdev })
    }

    pub fn device_id(&mut self) -> Result<u8, T::Error> {
        self.i2cdev.smbus_read_byte_data(KXTJ3_WHO_AM_I)
    }
}

impl<T> Accelerometer for KXTJ3Accelerometer<T>
where
    T: I2CDevice + Sized,
{
    type Error = T::Error;

    fn accelerometer_sample(&mut self) -> Result<AccelerometerSample, T::Error> {
        // datasheet recommends multi-byte read to avoid reading
        // an inconsistent set of data
        let mut buf: [u8; 6] = [0u8; 6];
        self.i2cdev.write(&[REGISTER_X0])?;
        self.i2cdev.read(&mut buf)?;

        let x: i16 = LittleEndian::read_i16(&[buf[0], buf[1]]);
        let y: i16 = LittleEndian::read_i16(&[buf[2], buf[3]]);
        let z: i16 = LittleEndian::read_i16(&[buf[4], buf[5]]);
        Ok(AccelerometerSample {
            x: (f32::from(x) / 1023.0) * (ACCEL_RANGE * 2.0),
            y: (f32::from(y) / 1023.0) * (ACCEL_RANGE * 2.0),
            z: (f32::from(z) / 1023.0) * (ACCEL_RANGE * 2.0),
        })
    }
}
