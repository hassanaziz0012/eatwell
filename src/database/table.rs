use postgres::Row;

pub trait Table<T> {
    // CREATE
    fn from_row(row: &Row) -> T;
    fn create(&self) -> Result<T, ErrorMsg>;

    // READ
    fn get(filters: Vec<Filter>) -> Result<T, ErrorMsg>;
    fn filter(filters: Vec<Filter>) -> Vec<T>;
    fn all() -> Vec<T>;

    // UPDATE
    fn save(&self) -> Result<T, ErrorMsg>;

    // DELETE
    fn delete(&self);

    fn test() {
        println!("Hello, World!");
    }
}

#[derive(Debug)]
pub struct ErrorMsg(pub String);

impl ErrorMsg {
    pub fn new(msg: String) -> ErrorMsg {
        ErrorMsg(msg)
    }
}

pub struct Filter {
    pub field: String,
    pub value: Value,
}

impl Filter {
    pub fn new(field: &str, value: Value) -> Filter {
        Filter { field: field.to_string(), value }
    }
}

pub enum Value {
    String(String),
    Int(i32),
}

impl Value {
    pub fn extract_int(&self) -> i32 {
        match self {
            Value::Int(i) => *i,
            _ => panic!("Value is not an int"),
        }
    }

    pub fn extract_string(&self) -> String {
        match self {
            Value::String(s) => s.to_string(),
            _ => panic!("Value is not a string"),
        }
    }
}