// use serde::{Deserialize, Serialize};
// use typeshare::typeshare;

// #[derive(Serialize, Deserialize, Clone)]
// #[typeshare]
// pub struct Locations {
//     pub id: i32,
//     pub name: String,
// }

// #[derive(Serialize, Deserialize)]
// #[typeshare]
// pub struct ShoppingList {
//     pub id: i32,
//     pub item: String,
//     pub location: i32,
// }

// // response that is returned from api calls
// // attempted to use enum instead but deserializing might be an issue?
// // if Error==True, Data *must* be None or otherwise ignored
// #[derive(Serialize, Deserialize)]
// #[typeshare]
// pub struct ServerResponse<T> {
//     pub error: bool,
//     pub message: Option<String>, // if error=true, contains the error message if any
//     pub data: Option<T>,
// }

// #[derive(Serialize, Deserialize, Clone, PartialEq)]
// #[typeshare]
// pub struct ShoppingData {
//     pub id: i32,
//     pub item: String,
//     pub location: String,
// }
