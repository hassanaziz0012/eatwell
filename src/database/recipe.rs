use std::str::FromStr;

use crate::{database::{self, table::Table, nutrition::Nutrition}, types::{diets::Diet, meal_types::MealTypes, cuisines::Cuisine}};
use postgres::Row;
use serde::{Serialize, Deserialize};

use super::table::ErrorMsg;

// TODO: Add the following fields: recipe instructions (prob new struct), diets, cuisines, dishTypes.
#[derive(Serialize, Deserialize)]
pub struct Recipe {
    pub id: i32,
    pub title: String,
    pub image: String,
    pub summary: String,
    #[serde(rename = "spoonacularSourceUrl")]
    pub spoonacular_source_url: String,
    #[serde(rename = "healthScore")]
    pub health_score: i32,
    #[serde(rename = "readyInMinutes")]
    pub ready_in_minutes: i32,
    #[serde(rename = "glutenFree")]
    pub gluten_free: bool,
    #[serde(rename = "dairyFree")]
    pub dairy_free: bool,
    pub cheap: bool,
    #[serde(rename = "veryHealthy")]
    pub very_healthy: bool,
    #[serde(rename = "veryPopular")]
    pub very_popular: bool,
    pub vegetarian: bool,
    pub servings: i32,
    pub diets: Vec<Diet>,
    pub cuisines: Vec<Cuisine>,
    #[serde(rename = "dishTypes")]
    pub meal_types: Vec<MealTypes>,
    #[serde(rename = "analyzedInstructions")]
    pub instructions: Vec<RecipeInstructions>,
    pub nutrition: Nutrition,
}

#[derive(Serialize, Deserialize)]
pub struct RecipeInstructions {
    pub name: String,
    pub steps: Vec<Step>
}

impl RecipeInstructions {
    pub fn new(name: String, steps: Vec<Step>) -> RecipeInstructions {
        RecipeInstructions { name, steps }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Step {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub number: i32,
    pub step: String,
}

impl Step {
    pub fn new(id: i32, number: i32, step: String) -> Step {
        Step { id, number, step }
    }
}

impl Recipe {
    pub fn new(
        title: String,
        image: String,
        summary: String,
        spoonacular_source_url: String,
        health_score: i32,
        ready_in_minutes: i32,
        gluten_free: bool,
        dairy_free: bool,
        cheap: bool,
        very_healthy: bool,
        very_popular: bool,
        vegetarian: bool,
        servings: i32,
        diets: Vec<Diet>,
        cuisines: Vec<Cuisine>,
        meal_types: Vec<MealTypes>,
        instructions: Vec<RecipeInstructions>,
        nutrition: Nutrition,
    ) -> Recipe {
        Recipe {
            id: 0,
            title,
            image,
            summary,
            spoonacular_source_url,
            health_score,
            ready_in_minutes,
            gluten_free,
            dairy_free,
            cheap,
            very_healthy,
            very_popular,
            vegetarian,
            servings,
            diets,
            cuisines,
            meal_types,
            instructions,
            // steps,
            nutrition,
        }
    }
    pub fn print_recipe(self: &Recipe) {
        println!("Recipe ID: {}", self.id);
        println!("Title: {}", self.title);
        println!("Image URL: {}", self.image);
        println!("Summary: {}", self.summary);
        println!("Source URL: {}", self.spoonacular_source_url);
        println!("Health Score: {}", self.health_score);
        println!("Ready in Minutes: {}", self.ready_in_minutes);
        println!("Is Gluten Free: {}", self.gluten_free);
        println!("Is Dairy Free: {}", self.dairy_free);
        println!("Is Cheap: {}", self.cheap);
        println!("Is Very Healthy: {}", self.very_healthy);
        println!("Is Very Popular: {}", self.very_popular);
        println!("Is Vegetarian: {}", self.vegetarian);
        println!("Servings: {}", self.servings);

        println!("Diets:");
        for diet in &self.diets {
            println!("  - {}", diet);
        }

        println!("Cuisines:");
        for cuisine in &self.cuisines {
            println!("  - {}", cuisine);
        }

        println!("Meal Types:");
        for meal_type in &self.meal_types {
            println!("  - {}", meal_type);
        }

        println!("Nutrition Information:");
        println!("Caloric Breakdown:");
        println!(
            "  - Percent Carbs: {}%",
            self.nutrition.caloric_breakdown.percent_carbs
        );
        println!(
            "  - Percent Fat: {}%",
            self.nutrition.caloric_breakdown.percent_fat
        );
        println!(
            "  - Percent Protein: {}%",
            self.nutrition.caloric_breakdown.percent_protein
        );

        println!("Nutrients:");
        for nutrient in &self.nutrition.nutrients {
            println!("  - Name: {}", nutrient.name);
            println!("    Amount: {} {}", nutrient.amount, nutrient.unit);
            println!(
                "    Percent of Daily Needs: {}%",
                nutrient.percent_of_daily_needs
            );
        }
    }

    pub fn get_connected_diets(recipe_id: i32) -> Vec<Diet> {
        let mut client = database::connect();
        let mut connected_diets = vec![];
        let query = "
            SELECT diet.id, diet.name FROM recipes AS recipe
            LEFT JOIN recipe_diets AS rd ON recipe.id = rd.recipe_id
            LEFT JOIN diets AS diet ON diet.id = rd.diet_id
            WHERE recipe.id = $1
        ";
        let params: &[&(dyn postgres::types::ToSql + std::marker::Sync)] = &[&recipe_id];
        let rows = client.query(query, params).unwrap();
        for row in rows {
            let name: String = row.get(1);
            let diet = Diet::from_str(&name).unwrap();
            connected_diets.push(diet);
        }
        connected_diets
    }

    pub fn get_connected_cuisines(recipe_id: i32) -> Vec<Cuisine> {
        let mut client = database::connect();
        let mut connected_cuisines = vec![];
        let query = "
            SELECT cuisine.id, cuisine.name FROM recipes AS recipe
            LEFT JOIN recipe_cuisines AS rc ON recipe.id = rc.recipe_id
            LEFT JOIN cuisines AS cuisine ON cuisine.id = rc.cuisine_id
            WHERE recipe.id = $1
        ";
        let params: &[&(dyn postgres::types::ToSql + std::marker::Sync)] = &[&recipe_id];
        let rows = client.query(query, params).unwrap();
        for row in rows {
            let name: String = row.get(1);
            let cuisine = Cuisine::from_str(&name).unwrap();
            connected_cuisines.push(cuisine);
        }
        connected_cuisines
    }

    pub fn get_connected_meal_types(recipe_id: i32) -> Vec<MealTypes> {
        let mut client = database::connect();
        let mut connected_meal_types = vec![];
        let query = "
            SELECT meal_type.id, meal_type.name FROM recipes AS recipe
            LEFT JOIN recipe_meal_types AS rmt ON recipe.id = rmt.recipe_id
            LEFT JOIN meal_types AS meal_type ON meal_type_id.id = rmt.meal_type_id
            WHERE recipe.id = $1
        ";
        let params: &[&(dyn postgres::types::ToSql + std::marker::Sync)] = &[&recipe_id];
        let rows = client.query(query, params).unwrap();
        for row in rows {
            let name: String = row.get(1);
            let meal_type = MealTypes::from_str(&name).unwrap();
            connected_meal_types.push(meal_type);
        }
        connected_meal_types
    }

    pub fn get_connected_steps(recipe_id: i32) -> Vec<Step> {
        let mut client = database::connect();
        let mut connected_steps = vec![];
        let query = "
            SELECT s.id, s.number, s.step FROM recipes AS recipe
            LEFT JOIN recipe_steps AS rs ON recipe.id = rs.recipe_id
            LEFT JOIN steps AS s ON s.id = rs.instruction_id
            WHERE recipe.id = $1
        ";
        let params: &[&(dyn postgres::types::ToSql + std::marker::Sync)] = &[&recipe_id];
        let rows = client.query(query, params).unwrap();
        for row in rows {
            let id: i32 = row.get(0);
            let number: i32 = row.get(1);
            let step: String = row.get(2);
            let obj: Step = Step::new(id, number, step);
            connected_steps.push(obj);
        }
        connected_steps
    }
}

impl Table<Recipe> for Recipe {
    fn from_row(row: &Row) -> Recipe {
        let id: i32 = row.get("id");
        let title: String = row.get("title");
        let image: String = row.get("image");
        let summary: String = row.get("summary");
        let spoonacular_source_url: String = row.get("spoonacular_source_url");
        let health_score: i32 = row.get("health_score");
        let ready_in_minutes: i32 = row.get("ready_in_minutes");
        let gluten_free: bool = row.get("gluten_free");
        let dairy_free: bool = row.get("dairy_free");
        let cheap: bool = row.get("cheap");
        let very_healthy: bool = row.get("very_healthy");
        let very_popular: bool = row.get("very_popular");
        let vegetarian: bool = row.get("vegetarian");
        let servings: i32 = row.get("servings");
        let nutrition_id: i32 = row.get("nutrition_id");

        let diets = Recipe::get_connected_diets(id);
        let cuisines = Recipe::get_connected_cuisines(id);
        let meal_types = Recipe::get_connected_meal_types(id);
        let steps = Recipe::get_connected_steps(id);
        let instructions = vec![RecipeInstructions::new("".to_string(), steps)];

        Recipe {
            id,
            title,
            image,
            summary,
            spoonacular_source_url,
            health_score,
            ready_in_minutes,
            gluten_free,
            dairy_free,
            cheap,
            very_healthy,
            very_popular,
            vegetarian,
            servings,
            diets,
            cuisines,
            meal_types,
            instructions,
            // steps: instructions.steps,
            nutrition: Nutrition::get(nutrition_id),
        }
    }

    fn create(self: &Recipe) -> Result<Recipe, ErrorMsg> {
        let mut client = database::connect();

        // Create recipe in database
        let query = "
            INSERT INTO recipes (
                title, 
                image, 
                summary, 
                spoonacular_source_url, 
                health_score, 
                ready_in_minutes, 
                gluten_free, 
                dairy_free, 
                cheap,
                very_healthy,
                very_popular,
                vegetarian,
                servings,
                diets,
                cuisines,
                meal_types,
                nutrition_id
            )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17
            )
            RETURNING *;
        ";
        let params: &[&(dyn postgres::types::ToSql + std::marker::Sync)] = &[
            &self.title,
            &self.image,
            &self.summary,
            &self.spoonacular_source_url,
            &self.health_score,
            &self.ready_in_minutes,
            &self.gluten_free,
            &self.dairy_free,
            &self.cheap,
            &self.very_healthy,
            &self.very_popular,
            &self.vegetarian,
            &self.servings,
            &self.nutrition.id,
        ];
        let row = client.query_one(query, params).unwrap();
        Ok(Recipe::from_row(&row))
    }

    fn save(self: &Recipe) -> Result<Recipe, ErrorMsg> {
        // TODO: To avoid duplicates each time you run the API, you need to do the following:
        // - When saving recipes from the API, check if they already exist. You can use the "title"
        // field since its unique. If it exists, just update its information from the API response.
        // Also update its nutrition information, etc.
        // - If the recipe does not exist, create it from scratch.
        // - Add some helper functions that will delete all the nutrients and nutrition rows that
        // are not connected to any recipe. This'll help keep the DB clean of unused data.

        // TODO: Finish adding CRUD operations. You need CREATE, READ, LIST, FILTER, UPDATE, and DELETE.
        let mut client = database::connect();

        // Check if recipe exists in database
        let query = "
            SELECT * FROM recipes WHERE title = $1;
        ";
        let params: &[&(dyn postgres::types::ToSql + std::marker::Sync)] = &[&self.title];
        match client.query_one(query, params) {
            Ok(row) => {
                // if exists
                // we use the recipe_id from database, because API returns its own ID which does not match our database.
                let recipe_id: i32 = row.get("id");
                // Update recipe
                let query = "
                    UPDATE recipes
                    SET 
                        title = $1, 
                        image = $2, 
                        summary = $3, 
                        spoonacular_source_url = $4, 
                        health_score = $5, 
                        ready_in_minutes = $6, 
                        gluten_free = $7, 
                        dairy_free = $8, 
                        cheap = $9,
                        very_healthy = $10,
                        very_popular = $11,
                        vegetarian = $12,
                        servings = $13
                    WHERE id = $10
                    RETURNING *;
                ";
                let params: &[&(dyn postgres::types::ToSql + std::marker::Sync)] = &[
                    &self.title,
                    &self.image,
                    &self.summary,
                    &self.spoonacular_source_url,
                    &self.health_score,
                    &self.ready_in_minutes,
                    &self.gluten_free,
                    &self.dairy_free,
                    &self.cheap,
                    &self.very_healthy,
                    &self.very_popular,
                    &self.vegetarian,
                    &self.servings,
                    &recipe_id
                ];
                let row = client.query_one(query, params).unwrap();
                Ok(Recipe::from_row(&row))
            }
            Err(_e) => {
                // if doesn't exist
                // Check if nutrition information exists in database
                let query = "
                    SELECT * FROM nutrition WHERE id = $1;
                ";
                let params: &[&(dyn postgres::types::ToSql + std::marker::Sync)] = &[&self.nutrition.id];
                match client.execute(query, params) {
                    Ok(_) => (),
                    Err(_e) => panic!("Nutrition for this recipe does not exist."),
                };

                // Create recipe in database
                let query = "
                    INSERT INTO recipes (
                        title, 
                        image, 
                        summary, 
                        spoonacular_source_url, 
                        health_score, 
                        ready_in_minutes, 
                        gluten_free, 
                        dairy_free, 
                        cheap,
                        very_healthy,
                        very_popular,
                        vegetarian,
                        servings,
                        nutrition_id
                    )
                    VALUES (
                        $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14
                    )
                    ON CONFLICT (title) DO UPDATE 
                    SET 
                        title = EXCLUDED.title,
                        image = EXCLUDED.image,
                        summary = EXCLUDED.summary,
                        spoonacular_source_url = EXCLUDED.spoonacular_source_url,
                        health_score = EXCLUDED.health_score,
                        ready_in_minutes = EXCLUDED.ready_in_minutes,
                        gluten_free = EXCLUDED.gluten_free,
                        dairy_free = EXCLUDED.dairy_free,
                        cheap = EXCLUDED.cheap,
                        very_healthy = EXCLUDED.very_healthy,
                        very_popular = EXCLUDED.very_popular,
                        vegetarian = EXCLUDED.vegetarian,
                        servings = EXCLUDED.servings,
                        nutrition_id = EXCLUDED.nutrition_id
                    RETURNING *;
                ";
                let params: &[&(dyn postgres::types::ToSql + std::marker::Sync)] = &[
                    &self.title,
                    &self.image,
                    &self.summary,
                    &self.spoonacular_source_url,
                    &self.health_score,
                    &self.ready_in_minutes,
                    &self.gluten_free,
                    &self.dairy_free,
                    &self.cheap,
                    &self.very_healthy,
                    &self.very_popular,
                    &self.vegetarian,
                    &self.servings,
                    &self.nutrition.id,
                ];
                let row = client.query_one(query, params).unwrap();
                Ok(Recipe::from_row(&row))
            }
        }
    }

    fn all() -> Vec<Recipe> {
        let mut client = database::connect();
        let query = "
            SELECT * FROM recipes;
        ";
        let rows = client.query(query, &[]).unwrap();
        let mut recipes = vec![];
        for row in rows {
            let recipe = Recipe::from_row(&row);
            recipes.push(recipe);
        }
        recipes
    }

    fn get(filters: Vec<database::table::Filter>) -> Result<Recipe, ErrorMsg> {
        let mut client = database::connect();
        // TODO: Handle filters here.
        let query = "
            SELECT * FROM recipes LIMIT 1;
        ";
        let params: &[&(dyn postgres::types::ToSql + std::marker::Sync)] = &[];
        let row = client.query_one(query, params).unwrap();
        Ok(Recipe::from_row(&row))
    }

    fn filter(filters: Vec<database::table::Filter>) -> Vec<Recipe> {
        let mut client = database::connect();
        // TODO: Handle filters here.
        let query = "
            SELECT * FROM recipes;
        ";
        let params: &[&(dyn postgres::types::ToSql + std::marker::Sync)] = &[];
        let rows = client.query(query, params).unwrap();
        let mut recipes = vec![];
        for row in rows {
            recipes.push(Recipe::from_row(&row));
        }
        recipes
    }

    fn delete(&self) {
        let mut client = database::connect();
        let query = "
            DELETE FROM recipes
            WHERE id = $1;
        ";
        let params: &[&(dyn postgres::types::ToSql + std::marker::Sync)] = &[&self.id];
        client.execute(query, params).unwrap();
    }
}