use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Deserialize, Serialize, Debug)]
pub enum Devices {
    SmartSocket(SmartSocket),
    SmartThermometer(SmartThermometer),
    SmartSpeaker(SmartSpeaker),
    SmartLamp(SmartLamp),
}

pub trait DeviceInfoProvider {
    fn get_device_status(&self, room: &str, name: &str) -> String;
}
#[derive(Clone, PartialEq, Eq, Deserialize, Serialize, Debug)]
pub struct SmartSocket {
    pub voltage: i32,
}

#[derive(Clone, PartialEq, Eq, Deserialize, Serialize, Debug)]
pub struct SmartThermometer {
    pub temperature: i32,
}
#[derive(Clone, PartialEq, Eq, Deserialize, Serialize, Debug)]
pub struct SmartSpeaker {
    pub volume: i32,
}
#[derive(Clone, PartialEq, Eq, Deserialize, Serialize, Debug)]
pub struct SmartLamp {
    pub is_enabled: bool,
}
