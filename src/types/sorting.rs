use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Sorting {
    Cholesterol,
    Phosphorus,
    MetaScore,
    TotalFat,
    Potassium,
    Popularity,
    Fluoride,
    Protein,
    Healthiness,
    TransFat,
    VitaminB2,
    Price,
    SaturatedFat,
    RiboFlavin,
    Time,
    MonoUnsaturatedFat,
    Selenium,
    Default,
    Random,
    PolyUnsaturatedFat,
    Sodium,
    Fiber,
    VitaminB1,
    Folate,
    Thiamin,
    Alcohol,
    FolicAcid,
    VitaminA,
    Caffiene,
    Iodine,
    VitaminB6,
    Copper,
    Iron,
    VitaminB12,
    Energy,
    Magnesium,
    VitaminC,
    Calories,
    Manganese,
    VitaminD,
    Calcium,
    VitaminB3,
    VitaminE,
    Carbohydrates,
    Niacin,
    VitaminK,
    Carbs,
    VitaminB5,
    Sugar,
    Choline,
    PanthothenicAcid,
    Zinc
}

impl Sorting {
    pub fn as_str(self: &Sorting) -> String {
        let val = serde_json::to_string(&self).unwrap();
        val.replace('"', "")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SortingDirection {
    Ascending,
    Descending
}

impl SortingDirection {
    pub fn as_str(self: &SortingDirection) -> &str {
        match self {
            SortingDirection::Ascending => "asc",
            SortingDirection::Descending => "desc"
        }
    }
}