use bcrypt::BcryptError;
use postgres::types::ToSql;
use sha2::{Sha256, Digest};
use uuid::Uuid;
use crate::database;

use super::{recipe::Recipe, table::{ErrorMsg, Filter, Table, Value}};

pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub auth_token: String,
    pub saved_recipes: Vec<Recipe>
}

impl User {
    pub fn new(name: String, email: String, password: String) -> User {
        User { id: 0, name, email, password, auth_token: User::generate_token(), saved_recipes: vec![] }
    }

    pub fn hash_password(password: &str) -> Result<String, BcryptError> {
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        let result = hasher.finalize();
        let password = format!("{:x}", result);
        Ok(password)
        // bcrypt::hash(password, bcrypt::DEFAULT_COST)
    }

    fn generate_token() -> String {
        Uuid::new_v4().to_string()
    }

    pub fn login(email: String, password: String) -> Result<User, ErrorMsg> {
        let hashed_pass = User::hash_password(password.as_str()).unwrap();
        let filters = vec![
            Filter::new("email", Value::String(email)),
        ];
        User::get(filters)
    }

    pub fn get_connected_recipes(id: i32) -> Vec<Recipe> {
        let mut client = database::connect();
        let query = "
            SELECT (
                r.id, 
                r.title, 
                r.image, 
                r.summary, 
                r.spoonacular_source_url, 
                r.health_score, 
                r.ready_in_minutes, 
                r.gluten_free, 
                r.dairy_free, 
                r.cheap, 
                r.nutrition_id
            ) FROM users AS u
            LEFT JOIN saved_recipes AS sr ON u.id = sr.user_id
            LEFT JOIN recipes AS r ON sr.recipe_id = r.id
            WHERE u.id = $1
        ";
        panic!();
    }

    pub fn add_recipe(self: &User, recipe: &Recipe) {
        let mut client = database::connect();
        let query = "
            INSERT INTO users_recipes
            (user_id, recipe_id)
            VALUES
            ($1, $2);
        ";
        let params: &[&(dyn postgres::types::ToSql + std::marker::Sync)] =
            &[&self.id, &recipe.id];
        client.execute(query, params).unwrap();
    }

    pub fn remove_recipe(self: &User, recipe: &Recipe) {
        let mut client = database::connect();
        let query = "
            DELETE FROM users_recipes
            WHERE
            user_id = $1 AND recipe_id = $2;
        ";
        let params: &[&(dyn postgres::types::ToSql + std::marker::Sync)] =
            &[&self.id, &recipe.id];
        client.execute(query, params).unwrap();
    }

    pub fn clear_recipes(self: &User) {
        let mut client = database::connect();
        let query = "
            DELETE FROM users_recipes
            WHERE
            user_id = $1;
        ";
        let params: &[&(dyn postgres::types::ToSql + std::marker::Sync)] = &[&self.id];
        client.execute(query, params).unwrap();
    }

    pub fn set_recipes(self: &User, recipes: &Vec<Recipe>) {
        self.clear_recipes();

        for recipe in recipes {
            self.add_recipe(recipe);
        }
    }
}

impl Table<User> for User {
    fn from_row(row: &postgres::Row) -> User {
        let id: i32 = row.get(0);
        let name: String = row.get(1);
        let email: String = row.get(2);
        let password: String = row.get(3);
        let auth_token: String = row.get(4);

        User { id, name, email, password, auth_token, saved_recipes: vec![] }
    }
    
    fn all() -> Vec<User> {
        let mut client = super::connect();
        let query = "
            SELECT * FROM users;
        ";
        let rows = client.query(query, &[]).unwrap();
        let mut users = vec![];
        for row in rows {
            users.push(User::from_row(&row));
        }
        users
    }

    fn filter(filters: Vec<super::table::Filter>) -> Vec<User> {
        let mut client = super::connect();
        let query = "
            SELECT * FROM users;
        ";
        let rows = client.query(query, &[]).unwrap();
        let mut users = vec![];
        for row in rows {
            users.push(User::from_row(&row));
        }
        users
    }

    fn get(filters: Vec<super::table::Filter>) -> Result<User, ErrorMsg> {
        let mut client = super::connect();

        let mut query = "
            SELECT * FROM USERS WHERE 
        ".to_string();
        let mut params = vec![];

        for (i, filter) in filters.iter().enumerate() {
            if filter.field == "id" {
                query.push_str(&format!("id = ${} AND ", i+1));
                params.push(&filter.value);
            }
            if filter.field == "name" {
                query.push_str(&format!("name = ${} AND ", i+1));
                params.push(&filter.value);
            }
            if filter.field == "email" {
                query.push_str(&format!("email = ${} AND ", i+1));
                params.push(&filter.value);
            }
            // if filter.field == "password" {
            //     query.push_str(&format!("password = ${} AND ", i+1));
            //     params.push(&filter.value);
            // }
        }
        query = query[..query.len()-4].to_string();
        query.push(';');
        let params = params.iter().map(|param| {
            match param {
                Value::String(s) => s as &(dyn ToSql + Sync),
                Value::Int(i) => i as &(dyn ToSql + Sync)
            }
        }).collect::<Vec<&(dyn ToSql + Sync)>>();

        let row = match client.query_one(query.as_str(), params.as_slice()) {
            Ok(row) => row,
            Err(e) => {
                if e.to_string().contains("unexpected number of rows") {
                    return Err(ErrorMsg("User not found".to_string()));
                }
                panic!("Error getting user: {}", e);
            },
        };
        Ok(User::from_row(&row))
    }

    fn create(&self) -> Result<User, ErrorMsg> {
        let mut client = super::connect();
        let query = "
            INSERT INTO users (
                name, 
                email, 
                password,
                auth_token
            ) 
            VALUES ($1, $2, $3, $4) 
            RETURNING *;
        ";
        let params: &[&(dyn postgres::types::ToSql + std::marker::Sync)] = &[
            &self.name,
            &self.email,
            &self.password,
            &self.auth_token
        ];
        match client.query_one(query, params) {
            Ok(row) => {
                let user = User::from_row(&row);
                user.set_recipes(&self.saved_recipes);
                Ok(user)
            },
            Err(e) => {
                let constraint = e.as_db_error().unwrap().constraint().unwrap();
                let code = e.code().unwrap().code();
                if constraint == "users_email_key" && code == "23505" {
                    let msg = format!("User with this email already exists ({})", self.email);
                    return Err(ErrorMsg::new(msg));
                }
                Err(ErrorMsg::new(format!("Error creating user: {}", e)))
            }
        }
    }

    fn save(&self) -> Result<User, ErrorMsg> {
        let mut client = super::connect();

        // TODO: Try to follow same standard on all DB tables. Use id=0 for unsaved objects, 
        // create() will create new object and save() will update them if they exist (id > 1) 
        // otherwise run create() to create them. That's it. No other complicated stuff necessary.
        if self.id > 0 {
            let query = "
                UPDATE users 
                SET
                    name = $1,
                    email = $2,
                    password = $3,
                    auth_token = $4
                WHERE id = $5
                RETURNING *;
            ";
            let params: &[&(dyn postgres::types::ToSql + std::marker::Sync)] = &[
                &self.name,
                &self.email,
                &self.password,
                &self.auth_token,
                &self.id
            ];
            let row = match client.query_one(query, params) {
                Ok(row) => row,
                Err(e) => {
                    panic!("Error saving user: {}", e)
                }
            };
            let user = User::from_row(&row);
            user.set_recipes(&self.saved_recipes);
            Ok(user)
        } else {
            self.create()
        }
    }

    fn delete(&self) {
        let mut client = super::connect();
        let query = "
            DELETE FROM users WHERE id = $1;
        ";
        let params: &[&(dyn postgres::types::ToSql + std::marker::Sync)] = &[&self.id];
        client.execute(query, params).unwrap();
    }
}