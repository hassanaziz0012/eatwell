use postgres::Row;
use serde::{Serialize, Deserialize};
use crate::database::{self, table::Table, nutrient::Nutrient};

use super::table::ErrorMsg;

#[derive(Serialize, Deserialize)]
pub struct Nutrition {
    #[serde(skip_deserializing)]
    pub id: i32,
    #[serde(rename = "caloricBreakdown")]
    pub caloric_breakdown: CaloricBreakdown,
    pub nutrients: Vec<Nutrient>,
}

impl Nutrition {
    pub fn get(id: i32) -> Nutrition {
        let mut client = database::connect();
        let query = "
            SELECT * FROM nutrition WHERE id = $1;
        ";
        let params: &[&(dyn postgres::types::ToSql + std::marker::Sync)] = &[&id];
        let row = client.query_one(query, params).unwrap();
        Nutrition::from_row(&row)
    }

    pub fn new(
        id: Option<i32>,
        caloric_breakdown: CaloricBreakdown,
        nutrients: Vec<Nutrient>,
    ) -> Nutrition {
        Nutrition {
            id: id.unwrap_or(0),
            caloric_breakdown,
            nutrients,
        }
    }

    /// Retrieves the connected Nutrients of a given NutritionInformation object.
    ///
    /// Connects the nutrition_nutrients table via a LEFT JOIN and parses the connected
    /// nutrients, then returns them in a Vec<Nutrient>.
    fn get_connected_nutrients(id: i32) -> Vec<Nutrient> {
        let mut client = database::connect();
        let query = "
            SELECT nt.id, nt.amount, nt.name, nt.unit, nt.percent_of_daily_needs FROM nutrition as n
            LEFT JOIN nutrition_nutrients AS nn ON n.id = nn.nutrition_id
            LEFT JOIN nutrients AS nt ON nn.nutrient_id = nt.id
            WHERE n.id = $1
        ";
        let params: &[&(dyn postgres::types::ToSql + std::marker::Sync)] = &[&id];
        let rows = client.query(query, params).unwrap();
        let mut nutrients = vec![];
        for row in rows {
            let id: i32 = row.get(0);
            let amount: f32 = row.get(1);
            let name: String = row.get(2);
            let unit: String = row.get(3);
            let percent_of_daily_needs: f32 = row.get(4);

            let nutrient = Nutrient::new(Some(id), amount, name, unit, percent_of_daily_needs);
            nutrients.push(nutrient);
        }
        nutrients
    }

    pub fn add_nutrient(self: &Nutrition, nutrient: &Nutrient) {
        let mut client = database::connect();
        let query = "
            INSERT INTO nutrition_nutrients
            (nutrition_id, nutrient_id)
            VALUES
            ($1, $2);
        ";
        let params: &[&(dyn postgres::types::ToSql + std::marker::Sync)] =
            &[&self.id, &nutrient.id];
        client.execute(query, params).unwrap();
    }

    pub fn remove_nutrient(self: &Nutrition, nutrient: &Nutrient) {
        let mut client = database::connect();
        let query = "
            DELETE FROM nutrition_nutrients
            WHERE
            nutrition_id = $1 AND nutrient_id = $2;
        ";
        let params: &[&(dyn postgres::types::ToSql + std::marker::Sync)] =
            &[&self.id, &nutrient.id];
        client.execute(query, params).unwrap();
    }

    pub fn clear_nutrients(self: &Nutrition) {
        let mut client = database::connect();
        let query = "
            DELETE FROM nutrition_nutrients
            WHERE
            nutrition_id = $1;
        ";
        let params: &[&(dyn postgres::types::ToSql + std::marker::Sync)] = &[&self.id];
        client.execute(query, params).unwrap();
    }

    pub fn set_nutrients(self: &Nutrition, nutrients: &Vec<Nutrient>) {
        self.clear_nutrients();

        for nutrient in nutrients {
            self.add_nutrient(nutrient);
        }
    }
}

impl Table<Nutrition> for Nutrition {
    /// Creates a NutritionInformation object from a postgres::Row object.
    ///
    /// Reads the caloric breakdown from the row, and retrieves connected nutrients
    /// separately from the database.
    ///
    /// Returns a complete NutritionInformation object.
    fn from_row(row: &Row) -> Nutrition {
        let id: i32 = row.get(0);
        let percent_carbs: f32 = row.get(1);
        let percent_fat: f32 = row.get(2);
        let percent_protein: f32 = row.get(3);

        let caloric_breakdown = CaloricBreakdown::new(percent_carbs, percent_fat, percent_protein);

        // Get connected nutrients from database
        let nutrients = Nutrition::get_connected_nutrients(id);

        Nutrition {
            id,
            caloric_breakdown,
            nutrients,
        }
    }

    fn get(filters: Vec<database::table::Filter>) -> Result<Nutrition, ErrorMsg> {
        let mut client = database::connect();
        let query = "
            SELECT * FROM nutrition LIMIT 1;
        ";
        let params: &[&(dyn postgres::types::ToSql + std::marker::Sync)] = &[];
        let row = client.query_one(query, params).unwrap();
        Ok(Nutrition::from_row(&row))
    }

    fn filter(filters: Vec<database::table::Filter>) -> Vec<Nutrition> {
        let mut client = database::connect();
        let query = "
            SELECT * FROM nutrition;
        ";
        let params: &[&(dyn postgres::types::ToSql + std::marker::Sync)] = &[];
        let rows = client.query(query, params).unwrap();
        let mut nutritions = vec![];
        for row in rows {
            nutritions.push(Nutrition::from_row(&row));
        }
        nutritions
    }

    fn all() -> Vec<Nutrition> {
        let mut client = database::connect();
        let query = "
            SELECT * FROM nutrition;
        ";
        let params: &[&(dyn postgres::types::ToSql + std::marker::Sync)] = &[];
        let rows = client.query(query, params).unwrap();
        let mut nutritions = vec![];
        for row in rows {
            nutritions.push(Nutrition::from_row(&row));
        }
        nutritions
    }

    /// Save the NutritionInformation object.
    ///
    /// If it exists in database, update its values. Otherwise, create a new object and save in database.
    fn save(self: &Nutrition) -> Result<Nutrition, ErrorMsg> {
        // If ID is > 0, the NutritionInformation object exists in the database. In that case, we will
        // simply update the values. Otherwise, we create a new object.
        if self.id > 0 {
            let mut client = database::connect();
            // Update nutrition information in database
            let query = "
                UPDATE nutrition
                SET 
                    percent_carbs = $1,
                    percent_fat = $2,
                    percent_protein = $3
                WHERE
                    id = $4
                RETURNING *;
            ";
            let params: &[&(dyn postgres::types::ToSql + std::marker::Sync)] = &[
                &self.caloric_breakdown.percent_carbs,
                &self.caloric_breakdown.percent_fat,
                &self.caloric_breakdown.percent_protein,
                &self.id,
            ];
            let row = client.query_one(query, params).unwrap();

            // Create a NutritionInformation object out of the new, updated information
            let nutrition_id: i32 = row.get(0);
            let percent_carbs: f32 = row.get(1);
            let percent_fat: f32 = row.get(2);
            let percent_protein: f32 = row.get(3);

            let caloric_breakdown =
                CaloricBreakdown::new(percent_carbs, percent_fat, percent_protein);

            // Get connected nutrients from database
            let nutrients = Nutrition::get_connected_nutrients(nutrition_id);

            let nutrition = Nutrition {
                id: nutrition_id,
                caloric_breakdown,
                nutrients,
            };
            Ok(nutrition)
        } else {
            let nutrition = Nutrition::new(None, self.caloric_breakdown, self.nutrients.clone());
            nutrition.create()
        }
    }

    /// Creates a new NutritionInformation object and saves it in the database.
    ///
    /// First creates the caloric breakdown data, and then connects the
    /// Nutrient objects to the NutritionInformation object.
    fn create(&self) -> Result<Nutrition, ErrorMsg> {
        let mut client = database::connect();
        // Create nutrition information in database
        let query = "
            INSERT INTO nutrition 
            (percent_carbs, percent_fat, percent_protein)
            VALUES
            ($1, $2, $3)
            RETURNING *;
        ";
        let params: &[&(dyn postgres::types::ToSql + std::marker::Sync)] = &[
            &self.caloric_breakdown.percent_carbs,
            &self.caloric_breakdown.percent_fat,
            &self.caloric_breakdown.percent_protein,
        ];
        let row = client.query_one(query, params).unwrap();
        let id: i32 = row.get(0);

        let nutrition = Nutrition {
            id,
            caloric_breakdown: self.caloric_breakdown,
            nutrients: self.nutrients.clone(),
        };
        Ok(nutrition)
    }

    fn delete(&self) {
        let mut client = database::connect();
        let query = "
            DELETE FROM nutrition WHERE id = $1;
        ";
        let params: &[&(dyn postgres::types::ToSql + std::marker::Sync)] = &[&self.id];
        client.execute(query, params).unwrap();
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct CaloricBreakdown {
    #[serde(rename = "percentCarbs")]
    pub percent_carbs: f32,
    #[serde(rename = "percentFat")]
    pub percent_fat: f32,
    #[serde(rename = "percentProtein")]
    pub percent_protein: f32,
}

impl CaloricBreakdown {
    pub fn new(percent_carbs: f32, percent_fat: f32, percent_protein: f32) -> CaloricBreakdown {
        CaloricBreakdown {
            percent_carbs,
            percent_fat,
            percent_protein,
        }
    }
}
