mod tables;
mod nutrient;
mod nutrition;
pub mod recipe;
pub mod table;
pub mod user;

use postgres::{Client, NoTls};
use tables::{create_nutrients_table, create_recipe_table, create_nutrition_table, create_nutrition_nutrients_table};

use self::tables::{create_cuisines_table, create_diets_table, create_meal_types_table, create_recipe_cuisines_table, create_recipe_diets_table, create_recipe_meal_types_table, create_users_recipes_table, create_users_table, setup_database_user};

pub fn create_database(client: &mut Client) {
    setup_database_user(client);
    create_nutrition_table(client);
    create_recipe_table(client);
    create_users_table(client);
    create_users_recipes_table(client);
    create_nutrients_table(client);
    create_nutrition_nutrients_table(client);
    create_diets_table(client);
    create_recipe_diets_table(client);
    create_meal_types_table(client);
    create_recipe_meal_types_table(client);
    create_cuisines_table(client);
    create_recipe_cuisines_table(client);

}

pub fn connect() -> Client {
    let params = "host=localhost user=hassan password=hassan dbname=eatwell";
    match Client::connect(
        params,
        NoTls,
    ) {
        Ok(c) => c,
        Err(e) => {
            let db_error = e.to_string();
            println!("{}", db_error);
            panic!("{}", e)
        }
    }
}
