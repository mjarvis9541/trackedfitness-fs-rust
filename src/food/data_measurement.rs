use leptos::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum DataMeasurement {
    Grams,
    Milliliters,
    #[default]
    Servings,
}

impl fmt::Display for DataMeasurement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let measurement = match self {
            DataMeasurement::Grams => "g",
            DataMeasurement::Milliliters => "ml",
            DataMeasurement::Servings => "srv",
        };
        write!(f, "{}", measurement)
    }
}

impl FromStr for DataMeasurement {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "g" => Ok(DataMeasurement::Grams),
            "ml" => Ok(DataMeasurement::Milliliters),
            "srv" => Ok(DataMeasurement::Servings),
            _ => Err("unknown data measurement"),
        }
    }
}

impl From<String> for DataMeasurement {
    fn from(s: String) -> Self {
        match s.as_str() {
            "g" => DataMeasurement::Grams,
            "ml" => DataMeasurement::Milliliters,
            "srv" => DataMeasurement::Servings,
            _ => panic!("unknown data measurement: {}", s),
        }
    }
}

impl DataMeasurement {
    pub fn to_form_step(&self) -> f64 {
        match self {
            DataMeasurement::Servings => 0.1,
            _ => 1.0,
        }
    }

    pub fn to_form_value(&self, quantity: RwSignal<Decimal>) -> String {
        match self {
            DataMeasurement::Servings => quantity.with(|x| format!("{:.1}", x)),
            _ => quantity.with(|x| format!("{:.0}", x)),
        }
    }

    pub fn to_quantity_modifier(&self, quantity: &Decimal) -> Decimal {
        match self {
            DataMeasurement::Grams | DataMeasurement::Milliliters => quantity * Decimal::new(1, 2),
            DataMeasurement::Servings => quantity * Decimal::new(1, 0),
        }
    }

    pub fn get_quantity_value(&self) -> Decimal {
        match self {
            DataMeasurement::Grams | DataMeasurement::Milliliters => Decimal::from(100),
            DataMeasurement::Servings => Decimal::from(1),
        }
    }

    pub fn to_data_value(&self) -> i32 {
        match self {
            DataMeasurement::Grams | DataMeasurement::Milliliters => 100,
            DataMeasurement::Servings => 1,
        }
    }
}
