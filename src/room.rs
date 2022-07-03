use crate::devices::Devices;
use std::collections::BTreeMap;

pub struct Room {
    pub devices: BTreeMap<&'static str, Devices>,
}
