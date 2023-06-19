use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
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

// response that is returned from api calls
// attempted to use enum instead but deserializing might be an issue?
// if Error==True, Data *must* be None or otherwise ignored
#[derive(Serialize, Deserialize)]
pub struct ServerResponse<T> {
    pub error: bool,
    pub message: Option<String>, // if error=true, contains the error message if any
    pub data: Option<T>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct ShoppingData {
    pub id: i32,
    pub item: String,
    pub location: String,
}

impl Locations {
    pub fn default() -> Self {
        Self {
            id: 0,
            name: String::from(""),
        }
    }
}
