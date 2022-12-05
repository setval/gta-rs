#![allow(dead_code)]
use crate::entity::Entity;

#[derive(Debug)]
pub struct Physical {
    address: usize,
    pub entity: Entity,
}

impl Physical {
    pub fn new(address: usize) -> Self {
        Self {
            address,
            entity: Entity::new(address),
        }
    }
}