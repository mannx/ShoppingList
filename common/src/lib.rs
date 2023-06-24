use serde::{Deserialize, Serialize};

pub mod server_response;

#[derive(Serialize, Deserialize, Clone, Debug)]
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
// #[derive(Serialize, Deserialize, Clone)]
// pub struct ServerResponse<T> {
//     pub error: bool,
//     pub message: Option<String>, // if error=true, contains the error message if any
//     pub data: Option<T>,
// }

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct ShoppingData {
    pub id: i32,
    pub item: String,
    pub location: String,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct NewItem {
    pub store_id: i32,
    pub item_name: String,
}

impl Locations {
    pub fn default() -> Self {
        Self {
            id: 0,
            name: String::from(""),
        }
    }
}

// impl<T> ServerResponse<T> {
//     pub fn new(error: bool, message: String) -> Self {
//         Self {
//             error,
//             message: Some(message),
//             data: None,
//         }
//     }
// }
