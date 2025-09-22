//! # The Fuel Monitor Library
//!
//! Tracks recent fuel levels

#![cfg_attr(not(test), no_std)]

pub const FUEL_LEVEL_MAX: FuelLevel = FuelLevel::with_litres(10.0);

/// All the ways this module can fail
#[derive(Clone, Copy, Debug, PartialEq, defmt::Format)]
pub enum Error {
    /// Tried to create a fuel level with a negative number
    NegativeFuelLevel,
    /// Tried to create NaN or Infinite fuel
    InvalidFuelLevel
}

/// Represents a fuel level in the tank
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct FuelLevel {
    level_litres: f64
}

impl defmt::Format for FuelLevel {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{} litres", self.level_litres);
    }
}

impl Eq for FuelLevel {

}

impl Ord for FuelLevel {
    fn cmp(&self, rhs: &Self) -> core::cmp::Ordering {
        if self.level_litres < rhs.level_litres {
            core::cmp::Ordering::Less
        } else if self.level_litres > rhs.level_litres {
            core::cmp::Ordering::Greater            
        } else {
            core::cmp::Ordering::Equal
        }
    }
}

impl FuelLevel {
    /// Create a new zero fuel level
    pub const fn zero() -> FuelLevel {
        FuelLevel { level_litres: 0.0 }
    }

    /// Create a new fuel level from a value in litres
    pub const fn with_litres(litres: f64) -> FuelLevel {
        if litres < 0.0 {
            panic!("Negative fuel level not supported");
        }
        if litres.is_nan() || litres.is_infinite() {
            panic!("Float value is illegal");
        }
        FuelLevel { level_litres: litres }
    }

    /// Create a new fuel level from a value in millilitres
    pub const fn with_millilitres(ml: f64) -> Result<FuelLevel, Error> {
        if ml < 0.0 {
            return Err(Error::NegativeFuelLevel);
        } else if ml.is_nan() || ml.is_infinite() {
            return Err(Error::InvalidFuelLevel);
        } else {
            Ok(FuelLevel { level_litres: ml / 1000.0 })
        }
    }

    pub const fn as_litres(self) -> f64 {
        self.level_litres
    }

    pub const fn as_millilitres(self) -> f64 {
        self.level_litres * 1000.0
    }
}

/// Tracks recentl fuel readings
pub struct FuelMonitor {
    levels: heapless::HistoryBuf<FuelLevel, 16>
}

impl FuelMonitor {
    /// Create a new fuel monitor with no readings
    pub fn new() -> FuelMonitor {
        FuelMonitor {
            levels: heapless::HistoryBuf::new()
        }
    }

    /// add a reading to the monitor
    pub fn insert(&mut self, level: FuelLevel) {
        self.levels.write(level);
    }

    /// Get the minimum fuel level
    pub fn min(&self) -> Option<FuelLevel> {
        self.levels.oldest_ordered().min().cloned()
    }

    /// Get the maximum fuel level
    pub fn max(&self) -> Option<FuelLevel> {
        self.levels.oldest_ordered().max().cloned()
    }

    /// Get the mean fuel level
    pub fn mean(&self) -> Option<FuelLevel> {
        if self.levels.len() == 0 {
            return None;
        }
        let mut total = 0.0;
        for level in self.levels.oldest_ordered() {
            total = total + level.as_litres();
        }
        Some(FuelLevel::with_litres(total / self.levels.len() as f64))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_level() {
        let zero = FuelLevel::zero();
        assert_eq!(zero.as_litres(), 0.0);
        assert_eq!(zero.as_millilitres(), 0.0);
    }

    #[test]
    #[should_panic]
    fn bad_fuel_level() {
        let _level = FuelLevel::with_litres(-1.0);
    }

    #[test]
    fn bad_fuel_level_result() {
        let level = FuelLevel::with_millilitres(-1000.0);
        assert_eq!(Err(Error::NegativeFuelLevel), level);
    }

    #[test]
    fn with_ml() {
        let level = FuelLevel::with_millilitres(1000.0);
        assert_eq!(level, Ok(FuelLevel::with_litres(1.0)));
    }

    #[test]
    fn insert_into_monitor() {
        let mut monitor = FuelMonitor::new();
        monitor.insert(FuelLevel::with_litres(1.0));
    }

    #[test]
    fn monitor_min() {
        let mut monitor = FuelMonitor::new();
        monitor.insert(FuelLevel::with_litres(1.0));
        monitor.insert(FuelLevel::with_litres(2.0));
        monitor.insert(FuelLevel::with_litres(3.0));
        assert_eq!(monitor.min(), Some(FuelLevel::with_litres(1.0)));
        assert_eq!(monitor.max(), Some(FuelLevel::with_litres(3.0)));
        assert_eq!(monitor.mean(), Some(FuelLevel::with_litres(2.0)));
    }
}
