mod database;
mod types;
mod web;
mod food_api;

use actix_web::web::Data;
use actix_web::{App, HttpServer};
use database::table::{Table, Filter};
use database::user::User;
use food_api::{Api, SearchFilters};
use web::AppState;
use web::routes::{echo, hello, search_recipes, signup, login};

use crate::database::create_database;
use crate::database::table::Value;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let api_key = "d5df4e90f51f401cb05b099de3d64b03";
        let base_url = "https://api.spoonacular.com";
        let client = Api::new(api_key, base_url);

        App::new()
            .app_data(Data::new(AppState::new(client)))
            .service(hello)
            .service(echo)
            .service(search_recipes)
            .service(signup)
            .service(login)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

// fn main() {
//     create_database(&mut database::connect());

//     // let mut user = User::new(
//     //     String::from("Hassan Aziz"), 
//     //     String::from("test@email.com"), 
//     // User::hash_password("testuser").unwrap()
//     // );
//     // user.save().unwrap();
//     let filters = vec![
//         Filter::new("email", Value::String("test@email.com".to_string())),
//     ];
//     let mut user = User::get(filters).unwrap();

//     let api_key = "d5df4e90f51f401cb05b099de3d64b03";
//     let base_url = "https://api.spoonacular.com";
//     let client = Api::new(api_key, base_url);

//     let filters = SearchFilters::new("ice cream".to_string(), None, None, None, None, None, None, 3);
//     let recipes = client.search_recipes(filters);
//     for mut recipe in recipes {
//         let nutrition = recipe.nutrition.save().unwrap();
//         recipe.nutrition.id = nutrition.id;
//         recipe.save().unwrap();
//         user.saved_recipes.push(recipe);
//     }
//     user.save().unwrap();
//     println!("{}", user.saved_recipes[0].title);


//     // let pass = "randompass123";
//     // let hashed_pass = User::hash_password(pass).unwrap();
//     // let user = User::new(String::from("Hassan Aziz"), String::from("hassanaziz0012@gmail.com"), hashed_pass);
//     // let user_id_filter = Filter::new("email", Value::String("test3@email.com".to_string()));
//     // let user = User::get(vec![user_id_filter]).unwrap();
//     // user.save().unwrap();
//     // let user = User::login(String::from("test3@email.com"), String::from("testuser")).unwrap();
//     // println!("{}", user.name);
    
// }