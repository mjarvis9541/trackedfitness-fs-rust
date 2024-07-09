use derive_more::Display;
use rust_decimal::Decimal;
use std::str::FromStr;

#[derive(Debug, Display, Default)]
pub enum ActivityLevel {
    #[display(fmt = "Sedentary")]
    Sedentary,
    #[default]
    #[display(fmt = "Lightly Active")]
    LightlyActive,
    #[display(fmt = "Moderately Active")]
    ModeratelyActive,
    #[display(fmt = "Very Active")]
    VeryActive,
    #[display(fmt = "Extremely Active")]
    ExtremelyActive,
    #[display(fmt = "-")]
    Default,
}

impl FromStr for ActivityLevel {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SD" => Ok(ActivityLevel::Sedentary),
            "LA" => Ok(ActivityLevel::LightlyActive),
            "MA" => Ok(ActivityLevel::ModeratelyActive),
            "VA" => Ok(ActivityLevel::VeryActive),
            "EA" => Ok(ActivityLevel::ExtremelyActive),
            "-" => Ok(ActivityLevel::Default),
            _ => Err(()),
        }
    }
}

impl From<ActivityLevel> for &str {
    fn from(value: ActivityLevel) -> Self {
        match value {
            ActivityLevel::Sedentary => "SD",
            ActivityLevel::LightlyActive => "LA",
            ActivityLevel::ModeratelyActive => "MA",
            ActivityLevel::VeryActive => "VA",
            ActivityLevel::ExtremelyActive => "EA",
            ActivityLevel::Default => "-",
        }
    }
}

impl ActivityLevel {
    pub fn to_tdee_modifier(&self) -> Decimal {
        match self {
            ActivityLevel::Sedentary => Decimal::new(1200, 3),
            ActivityLevel::LightlyActive => Decimal::new(1375, 3),
            ActivityLevel::ModeratelyActive => Decimal::new(1550, 3),
            ActivityLevel::VeryActive => Decimal::new(1725, 3),
            ActivityLevel::ExtremelyActive => Decimal::new(1900, 3),
            ActivityLevel::Default => Decimal::new(1000, 3),
        }
    }

    pub fn all_variants() -> Vec<&'static str> {
        vec!["SD", "LA", "MA", "VA", "EA"]
    }

    pub fn to_form_options() -> Vec<(&'static str, &'static str)> {
        let options = [
            ("", "Select"),
            ("SD", "Sedentary - little or no exercise/desk job"),
            (
                "LA",
                "Lightly Active - light exercise/sports 1-3 days a week",
            ),
            (
                "MA",
                "Moderately Active - Moderate exercise/sports 3-5 days a week",
            ),
            ("VA", "Very Active - Heavy exercise/sports 6-7 days a week"),
            (
                "EA",
                "Extremely Active - Very heavy exercise/physical job/training twice a day",
            ),
        ];
        options.to_vec()
    }
}
