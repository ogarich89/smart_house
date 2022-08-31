use crate::Devices;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, PartialEq, Eq, Deserialize, Serialize, Debug)]
pub struct Room {
    pub devices: BTreeMap<String, Devices>,
}
