use crate::devices::Devices;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Rooms {
    Hall,
    Kitchen,
    Bedroom,
}

pub struct Room {
    pub devices: HashMap<&'static str, Devices>,
}
