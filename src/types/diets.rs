use std::{str::FromStr, fmt::Display, convert::Infallible};

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Diet {
    #[serde(rename = "gluten free")]
    GlutenFree,
    #[serde(rename = "dairy free")]
    DairyFree,
    Ketogenic,
    Vegetarian,
    LactoVegetarian,
    #[serde(rename = "lacto ovo vegetarian")]
    LactoOvoVegetarian,
    #[serde(rename = "ovo vegetarian")]
    OvoVegetarian,
    Vegan,
    #[serde(rename = "whole 30")]
    Whole30,
    #[serde(rename = "fodmap friendly")]
    FodmapFriendly,
    #[serde(alias = "pescetarian")]
    Pescatarian,
    #[serde(alias = "paleo")]
    Paleolithic,
    Primal,
}

impl Diet {
    pub fn as_str(self: &Diet) -> String {
        let val = serde_json::to_string(&self).unwrap();
        val.replace('"', "")
    }
}

impl FromStr for Diet {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = format!(r#""{}""#, s);
        let diet = serde_json::from_str(val.as_str()).unwrap();
        Ok(diet)
    }
}

impl Display for Diet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}