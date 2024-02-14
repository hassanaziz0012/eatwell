use crate::{database, types, web};

use database::recipe::Recipe;
use reqwest::blocking::{Client, Response};
use reqwest::{Method, Url};
use serde_json::Value;
use serde_with::{formats::CommaSeparator, StringWithSeparator};
use types::cuisines::Cuisine;
use types::diets::Diet;
use types::meal_types::MealTypes;
use types::sorting::{Sorting, SortingDirection};
use serde::{Deserialize, Serialize};


#[derive(Clone)]
pub struct Api {
    api_key: String,
    base_url: String,
}

#[serde_with::serde_as]
#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchFilters {
    query: String,
    #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, Cuisine>>")]
    cuisines: Option<Vec<Cuisine>>,
    #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, Cuisine>>")]
    exclude_cuisines: Option<Vec<Cuisine>>,
    #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, Diet>>")]
    diet: Option<Vec<Diet>>,
    // #[serde_as(as = "Option<StringWithSeparator::<CommaSeparator, MealTypes>>")]
    meal_type: Option<MealTypes>,
    sorting: Option<Sorting>,
    sort_direction: Option<SortingDirection>,
    num: i32,
}

impl SearchFilters {
    pub fn new(
        query: String,
        cuisines: Option<Vec<Cuisine>>,
        exclude_cuisines: Option<Vec<Cuisine>>,
        diet: Option<Vec<Diet>>,
        meal_type: Option<MealTypes>,
        sorting: Option<Sorting>,
        sort_direction: Option<SortingDirection>,
        num: i32,
    ) -> Self {
        SearchFilters {
            query,
            cuisines,
            exclude_cuisines,
            diet,
            meal_type,
            sorting,
            sort_direction,
            num,
        }
    }
}

impl Api {
    pub fn new(api_key: &str, base_url: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            base_url: base_url.to_string(),
        }
    }

    fn send_request(&self, url: Url, method: Method) -> Result<Response, reqwest::Error> {
        println!("Sending request to: {}", url);
        Client::new()
            .request(method, url)
            .header("x-api-key", self.api_key.as_str())
            .send()
    }

    pub fn search_recipes(&self, filters: SearchFilters) -> Vec<Recipe> {
        let endpoint = self.base_url.to_string() + "/recipes/complexSearch";
        let num_results = filters.num.to_string();
        let mut params = vec![
            ("query", filters.query),
            ("addRecipeInformation", "true".to_string()),
            ("addRecipeNutrition", "true".to_string()),
            ("number", num_results),
        ];
        if let Some(sorting) = filters.sorting {
            match sorting {
                Sorting::Default => {}
                _ => params.push(("sort", sorting.as_str())),
            }
        }
        if filters.sort_direction.is_some() {
            params.push((
                "sortDirection",
                filters.sort_direction.unwrap().as_str().to_string(),
            ));
        }

        if filters.diet.is_some() {
            let mut diet_str = String::new();
            for diet in filters.diet.as_ref().unwrap() {
                diet_str += format!("{},", diet.as_str()).as_str();
            }
            diet_str.pop();
            params.push(("diet", diet_str));
        }

        if filters.meal_type.is_some() {
            let val = filters.meal_type.as_ref().unwrap().as_str();
            params.push(("type", val));
        }

        if filters.cuisines.is_some() {
            let mut cuisines_str = String::new();
            for cuisine in filters.cuisines.as_ref().unwrap() {
                cuisines_str += format!("{},", cuisine.as_str()).as_str();
            }
            cuisines_str.pop();
            println!("{}", cuisines_str);
            params.push(("cuisine", cuisines_str));
        }

        let url = reqwest::Url::parse_with_params(&endpoint, params).unwrap();
        let result = self
            .send_request(url, Method::GET)
            .unwrap()
            .json::<Value>()
            .unwrap();
        let recipes_json = result.get("results").unwrap();
        serde_json::from_value::<Vec<Recipe>>(recipes_json.clone()).unwrap()
    }

    fn random_recipes(&self, num: i32) -> Vec<Recipe> {
        let endpoint = self.base_url.to_string() + "/recipes/random";
        let num_results = num.to_string();
        let params = [
            ("number", num_results),
            ("addRecipeInformation", "true".to_string()),
            ("addRecipeNutrition", "true".to_string()),
            ("includeNutrition", "true".to_string()),
        ];
        let url = reqwest::Url::parse_with_params(&endpoint, params).unwrap();
        let result = self
            .send_request(url, Method::GET)
            .unwrap()
            .json::<Value>()
            .unwrap();
        let recipes_json = result.get("recipes").unwrap();
        serde_json::from_value::<Vec<Recipe>>(recipes_json.clone()).unwrap()
    }
}
