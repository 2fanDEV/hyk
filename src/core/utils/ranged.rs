use std::ops::{Add, AddAssign, Sub, SubAssign};

use anyhow::{anyhow, Result};
use egui::emath::Numeric;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Ranged<T: Numeric> {
    value: T,
    start: T,
    end: T,
}

impl<T: Numeric + SubAssign + Sub + Add + AddAssign> Ranged<T> {
    pub fn new(value: T, start: T, end: T) -> Result<Self> {
        if value >= start && value <= end {
            return Ok(Self { value, start, end });
        }
        Err(anyhow!(
            "value of ranged number is outside of defined range"
        ))
    }

    pub fn add(&mut self, value: T)
    where
        <T as Add>::Output: PartialOrd<T>,
    {
        if self.value + value < self.end {
            self.value += value;
        }
    }

    pub fn subtract(&mut self, value: T)
    where
        <T as Sub>::Output: PartialOrd<T>,
    {
        if self.value - value > self.start {
            self.value -= value;
        }
    }

    pub fn get(&mut self) -> T {
        self.value
    }
}
