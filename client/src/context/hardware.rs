use std::fmt::Debug;
use std::sync::Arc;
use dht11::Dht11;
use lazy_static::lazy_static;
use rppal::{gpio, spi};
use rppal::gpio::{Gpio, IoPin, OutputPin};
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use crate::context;
use crate::convert::IntoEnum;

#[derive(Clone, Debug)]
struct SpiDescriptor {
    bus: Bus,
    slave_select: SlaveSelect,
    clock_freq: u32,
    mode: Mode
}

pub struct Hardware {
    dht11_pin: u8,
    moisture_sensor: SpiDescriptor,
    moisture_sensor_calibration: usize,
    solenoid_valve_pin: u8
}

pub fn get() -> Arc<Hardware> {
    lazy_static! {
        static ref HARDWARE: Arc<Hardware> = Arc::new(Hardware::new());
    }
    return HARDWARE.clone()
}

impl Hardware {
    fn new() -> Self {
        let moisture_sensor = SpiDescriptor {
            bus: context::num_var::<u64>("MS_SPI_BUS").cast(),
            slave_select: context::num_var::<u64>("MS_SPI_SS").cast(),
            clock_freq: context::num_var("MS_SPI_CLK"),
            mode: context::num_var::<u64>("MS_SPI_MODE").cast()
        };
        Self {
            dht11_pin: context::num_var("DHT11"),
            moisture_sensor,
            moisture_sensor_calibration: context::num_var("SEN_CALIBRATION_MAX"),
            solenoid_valve_pin: context::num_var("SOLENOID"),
        }
    }

    pub fn dht11(&self) -> gpio::Result<Dht11<IoPin>> {
        let gpio = Gpio::new()?;
        Ok(Dht11::new(gpio.get(self.dht11_pin)?.into_io(gpio::Mode::Input)))
    }

    pub fn moisture_sensor(&self) -> rppal::spi::Result<Spi> {
        let desc = &self.moisture_sensor;
        Spi::new(desc.bus, desc.slave_select, desc.clock_freq, desc.mode)
    }

    pub fn solenoid_valve(&self) -> gpio::Result<OutputPin> {
        let gpio = Gpio::new()?;
        Ok(gpio.get(self.solenoid_valve_pin)?.into_output())
    }

    pub fn moisture_sensor_get_normalized_value(&self) -> Result<f32, spi::Error> {
        let mut moisture_sensor = self.moisture_sensor()?;
        let mut buffer = [0u8; 128];
        let moisture = moisture_sensor.read(&mut buffer)?;
        Ok(moisture as f32 / (self.moisture_sensor_calibration as f32))
    }
}

unsafe impl Sync for Hardware {

}