#![allow(dead_code)]
use crate::placeable::Placeable;

#[derive(Debug)]
pub struct Entity {
    address: usize,
    pub placeable: Placeable,
}

impl Entity {
    pub fn new(address: usize) -> Self {
        Self {
            address,
            placeable: Placeable::new(address),
        }
    }
}