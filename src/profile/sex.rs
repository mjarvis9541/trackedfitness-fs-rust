use derive_more::Display;
use rust_decimal::Decimal;
use std::str::FromStr;

#[derive(Debug, Default)]
pub struct BmrModifier {
    pub sex_modifier: Decimal,
    pub weight_modifier: Decimal,
    pub height_modifier: Decimal,
    pub age_modifier: Decimal,
}

#[derive(Debug, Display, Default)]
pub enum Sex {
    #[display(fmt = "Male")]
    Male,
    #[display(fmt = "Female")]
    Female,
    #[default]
    #[display(fmt = "-")]
    Default,
}

impl FromStr for Sex {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "M" => Ok(Sex::Male),
            "F" => Ok(Sex::Female),
            "-" => Ok(Sex::Default),
            _ => Err(()),
        }
    }
}

impl From<Sex> for &str {
    fn from(value: Sex) -> Self {
        match value {
            Sex::Male => "M",
            Sex::Female => "F",
            Sex::Default => "-",
        }
    }
}

impl Sex {
    pub fn to_bmr_modifier(&self) -> BmrModifier {
        match self {
            Sex::Male => BmrModifier {
                sex_modifier: Decimal::new(88362, 3),
                weight_modifier: Decimal::new(13397, 3),
                height_modifier: Decimal::new(4799, 3),
                age_modifier: Decimal::new(5677, 3),
            },
            Sex::Female => BmrModifier {
                sex_modifier: Decimal::new(447593, 3),
                weight_modifier: Decimal::new(9247, 3),
                height_modifier: Decimal::new(3098, 3),
                age_modifier: Decimal::new(4330, 3),
            },
            Sex::Default => BmrModifier::default(),
        }
    }

    pub fn all_variants() -> Vec<&'static str> {
        vec!["M", "F"]
    }

    pub fn to_form_options() -> Vec<(&'static str, &'static str)> {
        let options = [("", "Select"), ("M", "Male"), ("F", "Female")];
        options.to_vec()
    }
}
