pub enum Devices {
    SmartSocket(SmartSocket),
    SmartThermometer(SmartThermometer),
    SmartSpeaker(SmartSpeaker),
    SmartLamp(SmartLamp),
}

pub trait DeviceInfoProvider {
    fn get_device_status(&self, room: &'static str, name: &'static str) -> String;
}

pub struct SmartSocket {
    pub voltage: i32,
}

pub struct SmartThermometer {
    pub temperature: i32,
}

pub struct SmartSpeaker {
    pub volume: i32,
}

pub struct SmartLamp {
    pub is_enabled: bool,
}
