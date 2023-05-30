use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Locations {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ShoppingList {
    pub id: i32,
    pub item: String,
    pub location: i32,
}

#[derive(Serialize, Deserialize)]
// pub enum ServerResponse<T> {
//     Error(String),
//     Ok(T),
// }

// response that is returned from api calls
// attempted to use enum instead but deserializing might be an issue?
// if Error==True, Data *must* be None or otherwise ignored
pub struct ServerResponse<T> {
    pub error: bool,
    pub message: Option<String>, // if error=true, contains the error message if any
    pub data: Option<T>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct ShoppingData {
    pub id: i32,
    pub item: String,
    pub name: String,
}
