use std::{str::FromStr, convert::Infallible, fmt::Display};

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Cuisine {
    African,
    Asian,
    American,
    #[serde(rename = "South American")]
    SouthAmerican,
    British,
    Cajun,
    Caribbean,
    Chinese,
    Creole,
    English,
    #[serde(rename = "Eastern European")]
    EasternEuropean,
    European,
    French,
    German,
    Greek,
    Indian,
    Irish,
    Italian,
    Japanese,
    Jewish,
    Korean,
    #[serde(rename = "Latin American")]
    LatinAmerican,
    Mediterranean,
    Mexican,
    #[serde(rename = "Middle Eastern")]
    MiddleEastern,
    Nordic,
    Scottish,
    Southern,
    Spanish,
    Thai,
    Vietnamese
}

impl Cuisine {
    pub fn as_str(self: &Cuisine) -> String {
        let val = serde_json::to_string(&self).unwrap();
        val.replace('"', "")
    }
}

impl FromStr for Cuisine {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = format!(r#""{}""#, s);
        let cuisine = serde_json::from_str(val.as_str()).unwrap();
        Ok(cuisine)
    }
}

impl Display for Cuisine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}