use anyhow::{Context, Result};
use log::{error, info, trace, warn};

use std::collections::HashMap;

#[derive(Debug, Clone)]
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

/// sensor的属性
/// 属性名皆为`String`类型
/// 属性值类型则不固定
#[derive(Debug, Clone)]
enum Attr {
    Uid(u64),
    Hwid(u8),
    Type(SensorType),
    SensorName(String),
    VendorName(String),
    Rates(Vec<f32>),
    Ranges(Vec<(f32, f32)>),
    Bias(Vec<f32>),
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
    // rate: HashMap<f32, u32>,
    listeners: HashMap<f32, u32>,
    attr: HashMap<String, Attr>,
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
            // rate: HashMap::new(),
            listeners: HashMap::new(),
            attr: HashMap::new(),
        }
    }

    fn open(&mut self, req_odr: f32) {
        //self.
    }

    fn publish_default_attributes(&mut self) {
        self.attr
            .entry(String::from("sensor_name"))
            .or_insert(Attr::SensorName(self.sensor_name.clone()));
        self.attr
            .entry(String::from("vendor_name"))
            .or_insert(Attr::VendorName(self.vendor_name.clone()));
        self.attr
            .entry(String::from("hw_idx"))
            .or_insert(Attr::Hwid(self.idx));
        self.attr
            .entry(String::from("sensor_type"))
            .or_insert(Attr::Type(self.sensor_type.clone()));
    }

    fn update_attributes(&mut self, attr_type: String, attr_value: Attr) {
        let val = self.attr.entry(attr_type).or_insert(attr_value.clone());
        *val = attr_value;
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
    bmi320.publish_default_attributes();
    bmi320.update_attributes(
        String::from("rates"),
        Attr::Rates(vec![
            1.0, 12.5, 25.0, 50.0, 100.0, 200.0, 400.0, 800.0, 1600.0,
        ]),
    );
    bmi320.update_attributes(
        String::from("ranges"),
        Attr::Ranges(vec![(-16.0, 16.0), (-8.0, 8.0), (-4.0, 4.0), (-2.0, 2.0)]),
    );

    bmi320.open(3 as f32);

    Ok(())
}
