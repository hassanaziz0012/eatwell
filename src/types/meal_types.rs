use std::{str::FromStr, fmt::Display, convert::Infallible};

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MealTypes {
    #[serde(rename = "main course", alias = "main dish")]
    MainCourse,
    #[serde(rename = "morning meal")]
    MorningMeal,
    Bread,
    Condiment,
    Dip,
    Marinade,
    #[serde(rename = "side dish")]
    SideDish,
    Breakfast,
    Fingerfood,
    Dessert,
    Starter,
    Soup,
    Snack,
    #[serde(alias = "antipasto")]
    Antipasti,
    Appetizer,
    Beverage,
    Dinner,
    Lunch,
    Brunch,
    Drink,
    #[serde(rename = "hor d'oeuvre")]
    Hordoeuvre,
    Spread,
    Salad,
    Sauce
}

impl MealTypes {
    pub fn as_str(self: &MealTypes) -> String {
        let val = serde_json::to_string(&self).unwrap();
        val.replace('"', "")
    }
}

impl FromStr for MealTypes {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = format!(r#""{}""#, s);
        let meal_type = serde_json::from_str(val.as_str()).unwrap();
        Ok(meal_type)
    }
}

impl Display for MealTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())   
    }
}