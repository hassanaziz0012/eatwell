use postgres::Row;
use serde::{Serialize, Deserialize};

use crate::database::{self, table::Table};

use super::table::ErrorMsg;

#[derive(Serialize, Deserialize, Clone)]
pub struct Nutrient {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub amount: f32,
    pub name: String,
    pub unit: String,
    #[serde(rename = "percentOfDailyNeeds")]
    pub percent_of_daily_needs: f32,
}

impl Nutrient {
    /// Creates a new Nutrient object with ID = 0 if unspecified.
    pub fn new(
        id: Option<i32>,
        amount: f32,
        name: String,
        unit: String,
        percent_of_daily_needs: f32,
    ) -> Nutrient {
        let id = id.unwrap_or(0);
        Nutrient {
            id,
            amount,
            name,
            unit,
            percent_of_daily_needs,
        }
    }
}

impl Table<Nutrient> for Nutrient {
    fn from_row(row: &Row) -> Nutrient {
        let id: i32 = row.get(0);
        let amount: f32 = row.get(1);
        let name: String = row.get(2);
        let unit: String = row.get(3);
        let percent_of_daily_needs: f32 = row.get(4);
        Nutrient::new(Some(id), amount, name, unit, percent_of_daily_needs)
    }
    
    fn all() -> Vec<Nutrient> {
        let mut client = database::connect();
        let query = "
            SELECT * FROM nutrients;
        ";
        let rows = client.query(query, &[]).unwrap();
        let mut nutrients = vec![];
        for row in rows {
            let nutrient = Nutrient::from_row(&row);
            nutrients.push(nutrient);
        }
        nutrients
    }

    fn get(filters: Vec<database::table::Filter>) -> Result<Nutrient, ErrorMsg> {
        let mut client = database::connect();
        let query = "
            SELECT * FROM nutrients LIMIT 1;
        ";
        let params: &[&(dyn postgres::types::ToSql + std::marker::Sync)] = &[];
        let row = client.query_one(query, params).unwrap();
        Ok(Nutrient::from_row(&row))
    }

    fn filter(filters: Vec<database::table::Filter>) -> Vec<Nutrient> {
        let mut client = database::connect();
        let query = "
            SELECT * FROM nutrients;
        ";
        let params: &[&(dyn postgres::types::ToSql + std::marker::Sync)] = &[];
        let rows = client.query(query, params).unwrap();
        let mut nutrients = vec![];
        for row in rows {
            let nutrient = Nutrient::from_row(&row);
            nutrients.push(nutrient);
        }
        nutrients
    }

    fn create(&self) -> Result<Nutrient, ErrorMsg> {
        let mut client = database::connect();

        let query = "
            INSERT INTO nutrients
            (amount, name, unit, percent_of_daily_needs)
            VALUES
            ($1, $2, $3, $4)
            RETURNING *;
        ";
        let params: &[&(dyn postgres::types::ToSql + std::marker::Sync)] = &[
            &self.amount,
            &self.name,
            &self.unit,
            &self.percent_of_daily_needs,
        ];
        let row = client.query_one(query, params).unwrap();
        let id: i32 = row.get(0);

        let nutrient = Nutrient {
            id,
            amount: self.amount,
            name: self.name.clone(),
            unit: self.unit.clone(),
            percent_of_daily_needs: self.percent_of_daily_needs,
        };
        Ok(nutrient)
    }

    /// Creates a new Nutrient object and saves it in the database.
    fn save(&self) -> Result<Nutrient, ErrorMsg> {
        let mut client = database::connect();
        if self.id > 0 {
            // exists
            let query = "
                UPDATE nutrients
                SET
                    amount = $1,
                    name = $2,
                    unit = $3,
                    percent_of_daily_needs = $4
                WHERE
                    id = $5
                RETURNING *;
            ";
            let params: &[&(dyn postgres::types::ToSql + std::marker::Sync)] = &[
                &self.amount,
                &self.name,
                &self.unit,
                &self.percent_of_daily_needs,
                &self.id,
            ];
            let row = client.query_one(query, params).unwrap();

            let id: i32 = row.get(0);
            let amount: f32 = row.get(1);
            let name: String = row.get(2);
            let unit: String = row.get(3);
            let percent_of_daily_needs: f32 = row.get(4);

            Ok(Nutrient::new(Some(id), amount, name, unit, percent_of_daily_needs))
        } else {
            let nutrient = Nutrient::new(
                None,
                self.amount,
                self.name.clone(),
                self.unit.clone(),
                self.percent_of_daily_needs,
            );
            nutrient.create()
        }
    }

    fn delete(&self) {
        let mut client = database::connect();
        let query = "
            DELETE FROM nutrients WHERE id = $1;
        ";
        let params: &[&(dyn postgres::types::ToSql + std::marker::Sync)] = &[&self.id];
        client.execute(query, params).unwrap();
    }
}
