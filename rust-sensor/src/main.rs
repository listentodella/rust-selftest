use anyhow::{Context, Result};
use log::{error, info, trace, warn};

use std::collections::HashMap;

#[derive(Debug)]
enum SensorType {
    Accelermeter,
    Gyroscope,
    Magnetic,
    Temperature,
    AmbientLight,
    Proximity,
}

enum Sensor {
    Accel(),
    Gyro(),
    Magnet(),
    Temperature(),
}

/// 物理传感器,指有实际器件的传感器
/// `idx`: 为该sensor隶属于的`SensorType`的序号
/// `sensor_name`: 该senso的名称,一般为型号名称
/// `vendor_name`: 该sensor的供应商名称
/// `sensor_type`: 该sensor的`SensorType`
/// `rate`: 该sensor支持的odr,
/// `listeners`: 该sensor的监听者请求的odr,并且记录了对应的odr的现存的请求者的数量
#[derive(Debug)]
struct PhysicalSensor {
    idx: u8,
    sensor_name: String,
    vendor_name: String,
    sensor_type: SensorType,
    rate: HashMap<f32, u32>,
    listeners: HashMap<f32, u32>,
}

impl PhysicalSensor {
    fn new(
        sensor_type: SensorType,
        idx: u8,
        sensor_name: String,
        vendor_name: String,
    ) -> PhysicalSensor {
        PhysicalSensor {
            idx,
            sensor_name,
            vendor_name,
            sensor_type,
            rate: HashMap::new(),
            listeners: HashMap::new(),
        }
    }

    fn open(&mut self, req_odr: f32) {
        self.
    }
}

fn main() -> Result<()> {
    env_logger::init();

    let mut bmi320 = PhysicalSensor::new(
        SensorType::Accelermeter,
        0,
        String::from("bmi320"),
        String::from("Bosch"),
    );

    info!("create sensor {:#?}", bmi320);

    bmi320.open(3);
    info!("create sensor {:#?}", bmi320);

    Ok(())
}
