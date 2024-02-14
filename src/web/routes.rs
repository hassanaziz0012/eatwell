use std::collections::HashMap;
use serde::{Serialize, Deserialize};

// TODO: Create backend endpoints for the following operations:
// Search recipes through API.
// Create, Read, List, Filter, Update, Delete recipes.
// Profile - user auth, library (saving recipes).
use actix_web::{
    get, post,
    web::{Data, Query, Json},
    HttpResponse, Responder,
};
use crate::{food_api::SearchFilters, database::{user::User, table::Table}};
use responses::{ResponseTypes, SignupErrorResponse, SignupSuccessResponse};

use crate::AppState;

#[get("/search-recipes")]
async fn search_recipes(state: Data<AppState>, params: Query<SearchFilters>) -> impl Responder {
    let search_results =
        match tokio::task::spawn_blocking(move || state.client.search_recipes(params.0)).await {
            Ok(recipes) => recipes,
            Err(e) => {
                panic!("{}", e.to_string());
            }
        };
    let resp = serde_json::to_string(&search_results).unwrap();
    HttpResponse::Ok().body(resp)
}

#[post("/recipe")]
async fn save_recipe() -> impl Responder {
    HttpResponse::Ok().body("Create Recipe")
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/signup")]
async fn signup(data: Json<request_data::UserSignup>) -> impl Responder {
    println!("Creating user...");
    let hashed_pass = User::hash_password(data.0.password.as_str()).unwrap();
    let user = User::new(data.0.name, data.0.email, hashed_pass);
    let result = tokio::task::spawn_blocking(move || user.save()).await.unwrap();
    
    let resp = match result {
        Ok(u) => {
            let resp = ResponseTypes::SignupSuccess(SignupSuccessResponse {
                success: true,
                email: u.email,
                auth_token: u.auth_token
            });
            resp.get_response()
        },
        Err(e) => {
            let resp = ResponseTypes::SignupError(SignupErrorResponse {
                success: false,
                error: e.0
            });
            resp.get_response()
        }
    };
    HttpResponse::Ok().body(resp)
}

#[post("/login")]
async fn login(data: Json<request_data::UserLogin>) -> impl Responder {
    println!("Logging in user...");
    let result = tokio::task::spawn_blocking(move || User::login(data.0.email, data.0.password)).await.unwrap();
    let resp = match result {
        Ok(u) => {
            let resp = ResponseTypes::SignupSuccess(SignupSuccessResponse {
                success: true,
                email: u.email,
                auth_token: u.auth_token
            });
            resp.get_response()
        },
        Err(e) => {
            let resp = ResponseTypes::SignupError(SignupErrorResponse {
                success: false,
                error: e.0
            });
            resp.get_response()
        }
    };
    HttpResponse::Ok().body(resp)
}

mod request_data {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct UserSignup {
        pub name: String,
        pub email: String,
        pub password: String
    }

    #[derive(Deserialize)]
    pub struct UserLogin {
        pub email: String,
        pub password: String
    }
}

pub mod responses {
    use serde::{Serialize, Deserialize};

    pub enum ResponseTypes {
        SignupSuccess(SignupSuccessResponse),
        SignupError(SignupErrorResponse)
    }

    impl ResponseTypes {
        pub fn get_response(self) -> String {
            match self {
                ResponseTypes::SignupSuccess(resp) => serde_json::to_string(&resp).unwrap(),
                ResponseTypes::SignupError(resp) => serde_json::to_string(&resp).unwrap(),
            }
            
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct SignupSuccessResponse {
        pub success: bool,
        pub email: String,
        pub auth_token: String
    }

    #[derive(Serialize, Deserialize)]
    pub struct SignupErrorResponse {
        pub success: bool,
        pub error: String
    }

}