use derive_more::Display;
use rust_decimal::Decimal;
use std::str::FromStr;

#[derive(Debug, Display, Default, Clone)]
pub enum FitnessGoal {
    #[display(fmt = "Lose Weight")]
    LoseWeight,
    #[display(fmt = "Gain Weight")]
    GainWeight,
    #[default]
    #[display(fmt = "Maintain Weight")]
    MaintainWeight,
    #[display(fmt = "-")]
    Default,
}

impl FromStr for FitnessGoal {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "lw" => Ok(FitnessGoal::LoseWeight),
            "gw" => Ok(FitnessGoal::GainWeight),
            "mw" => Ok(FitnessGoal::MaintainWeight),
            _ => Err(()),
        }
    }
}

impl From<FitnessGoal> for &str {
    fn from(value: FitnessGoal) -> Self {
        match value {
            FitnessGoal::LoseWeight => "LW",
            FitnessGoal::GainWeight => "GW",
            FitnessGoal::MaintainWeight => "MW",
            FitnessGoal::Default => "-",
        }
    }
}

impl FitnessGoal {
    pub fn to_calorie_modifier(&self) -> Decimal {
        match self {
            FitnessGoal::LoseWeight => Decimal::new(8, 1),
            FitnessGoal::GainWeight => Decimal::new(11, 1),
            FitnessGoal::MaintainWeight => Decimal::new(10, 1),
            FitnessGoal::Default => Decimal::new(10, 1),
        }
    }

    pub fn all_variants() -> Vec<&'static str> {
        vec!["LW", "MW", "GW"]
    }
}

#[cfg(feature = "ssr")]
#[derive(Debug)]
pub struct TargetModifier {
    pub energy_factor: Decimal,
    pub protein_pct: Decimal,
    pub carbohydrate_pct: Decimal,
    pub fat_pct: Decimal,
    pub saturates_pct: Decimal,
    pub sugars_pct: Decimal,
    pub fibre: Decimal,
    pub salt: Decimal,
}

#[cfg(feature = "ssr")]
impl From<FitnessGoal> for TargetModifier {
    fn from(value: FitnessGoal) -> Self {
        match value {
            FitnessGoal::LoseWeight => TargetModifier {
                energy_factor: Decimal::new(8, 1),
                protein_pct: Decimal::new(40, 2),
                carbohydrate_pct: Decimal::new(40, 2),
                fat_pct: Decimal::new(20, 2),
                saturates_pct: Decimal::new(35, 2),
                sugars_pct: Decimal::new(3, 2),
                fibre: Decimal::from(30),
                salt: Decimal::from(6),
            },
            FitnessGoal::GainWeight => TargetModifier {
                energy_factor: Decimal::new(11, 1),
                protein_pct: Decimal::new(25, 2),
                carbohydrate_pct: Decimal::new(55, 2),
                fat_pct: Decimal::new(20, 2),
                saturates_pct: Decimal::new(35, 2),
                sugars_pct: Decimal::new(3, 2),
                fibre: Decimal::from(30),
                salt: Decimal::from(6),
            },
            FitnessGoal::MaintainWeight | FitnessGoal::Default => TargetModifier {
                energy_factor: Decimal::new(10, 1),
                protein_pct: Decimal::new(25, 2),
                carbohydrate_pct: Decimal::new(55, 2),
                fat_pct: Decimal::new(20, 2),
                saturates_pct: Decimal::new(35, 2),
                sugars_pct: Decimal::new(3, 2),
                fibre: Decimal::from(30),
                salt: Decimal::from(6),
            },
        }
    }
}
