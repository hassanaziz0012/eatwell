use postgres::Client;

pub fn create_users_table(client: &mut Client) {
    let query = "
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name TEXT,
            email TEXT UNIQUE,
            password TEXT,
            auth_token TEXT UNIQUE
        );
    ";
    client.execute(query, &[]).unwrap();
}

pub fn create_recipe_table(client: &mut Client) {
    let query = "
        CREATE TABLE IF NOT EXISTS recipes (
            id SERIAL PRIMARY KEY,
            title TEXT UNIQUE,
            image TEXT,
            summary TEXT,
            spoonacular_source_url TEXT,
            health_score INTEGER,
            ready_in_minutes INTEGER,
            gluten_free BOOLEAN,
            dairy_free BOOLEAN,
            cheap BOOLEAN,
            very_healthy BOOLEAN,
            very_popular BOOLEAN,
            vegetarian BOOLEAN,
            servings INTEGER,
            nutrition_id INTEGER REFERENCES nutrition(id)
        );
    ";
    client.execute(query, &[]).unwrap();
}

pub fn create_users_recipes_table(client: &mut Client) {
    let query = "
        CREATE TABLE IF NOT EXISTS users_recipes (
            user_id INTEGER REFERENCES users(id),
            recipe_id INTEGER REFERENCES recipes(id),
            PRIMARY KEY (user_id, recipe_id)
        );
    ";
    client.execute(query, &[]).unwrap();
}

pub fn create_nutrition_table(client: &mut Client) {
    let query = "
        CREATE TABLE IF NOT EXISTS nutrition (
            id SERIAL PRIMARY KEY,
            percent_carbs REAL,
            percent_fat REAL,
            percent_protein REAL
        );
    ";
    client.execute(query, &[]).unwrap();
}

pub fn create_nutrients_table(client: &mut Client) {
    let query = "
        CREATE TABLE IF NOT EXISTS nutrients (
            id SERIAL PRIMARY KEY,
            amount REAL,
            name TEXT,
            unit TEXT,
            percent_of_daily_needs REAL
        );
    ";
    client.execute(query, &[]).unwrap();
}

pub fn create_nutrition_nutrients_table(client: &mut Client) {
    let query = "
        CREATE TABLE IF NOT EXISTS nutrition_nutrients (
            nutrition_id INTEGER REFERENCES nutrition(id),
            nutrient_id INTEGER REFERENCES nutrients(id),
            PRIMARY KEY (nutrition_id, nutrient_id)
        )
    ";
    client.execute(query, &[]).unwrap();
}

pub fn create_cuisines_table(client: &mut Client) {
    let query = "
        CREATE TABLE IF NOT EXISTS cuisines (
            id SERIAL PRIMARY KEY,
            name TEXT
        )
    ";
    client.execute(query, &[]).unwrap();
}

pub fn create_recipe_cuisines_table(client: &mut Client) {
    let query = "
        CREATE TABLE IF NOT EXISTS recipe_cuisines (
            recipe_id INTEGER REFERENCES recipes(id),
            cuisine_id INTEGER REFERENCES cuisines(id),
            PRIMARY KEY (recipe_id, cuisine_id)
        )
    ";
    client.execute(query, &[]).unwrap();
}

pub fn create_diets_table(client: &mut Client) {
    let query = "
        CREATE TABLE IF NOT EXISTS diets (
            id SERIAL PRIMARY KEY,
            name TEXT
        )
    ";
    client.execute(query, &[]).unwrap();
}

pub fn create_recipe_diets_table(client: &mut Client) {
    let query = "
        CREATE TABLE IF NOT EXISTS recipe_diets (
            recipe_id INTEGER REFERENCES recipes(id),
            diet_id INTEGER REFERENCES diets(id),
            PRIMARY KEY (recipe_id, diet_id)
        )
    ";
    client.execute(query, &[]).unwrap();
}

pub fn create_meal_types_table(client: &mut Client) {
    let query = "
        CREATE TABLE IF NOT EXISTS meal_types (
            id SERIAL PRIMARY KEY,
            name TEXT
        )
    ";
    client.execute(query, &[]).unwrap();
}

pub fn create_recipe_meal_types_table(client: &mut Client) {
    let query = "
        CREATE TABLE IF NOT EXISTS recipe_meal_types (
            recipe_id INTEGER REFERENCES recipes(id),
            meal_type_id INTEGER REFERENCES meal_types(id),
            PRIMARY KEY (recipe_id, meal_type_id)
        )
    ";
    client.execute(query, &[]).unwrap();
}

pub fn create_steps_table(client: &mut Client) {
    let query = "
        CREATE TABLE IF NOT EXISTS steps (
            id SERIAL PRIMARY KEY,
            number INTEGER,
            step TEXT
        )
    ";
    client.execute(query, &[]).unwrap();
}

pub fn create_recipe_steps_table(client: &mut Client) {
    let query = "
        CREATE TABLE IF NOT EXISTS recipe_steps (
            recipe_id INTEGER REFERENCES recipes(id),
            step_id INTEGER REFERENCES steps(id),
            PRIMARY KEY (recipe_id, step_id)
        )
    ";
    client.execute(query, &[]).unwrap();
}

pub fn setup_database_user(client: &mut Client) {
    let query = "
        GRANT ALL PRIVILEGES ON DATABASE eatwell TO hassan;
    ";
    client.execute(query, &[]).unwrap();
    let query = "
        GRANT ALL ON SCHEMA public TO hassan;
    ";
    client.execute(query, &[]).unwrap();
}