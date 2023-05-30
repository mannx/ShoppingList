// use crate::prelude::*;
// use yew::Properties;

// #[derive(Properties, PartialEq)]
// struct ShopDataProp {
//     location: String,
//     data: Vec<ShoppingData>,
// }

// pub fn view_data(data: &HashMap<String, Vec<ShoppingData>>) -> Html {
//     html! {<>
//     {data.iter().map(|(k,v)| html!{<ShopData location={k.clone()} data={v.clone()} />}).collect::<Vec<Html>>()}
//     </>}
// }

// #[function_component(ShopData)]
// fn shop_data(props: &ShopDataProp) -> Html {
//     html! {
//         <div><h3>{&props.location}</h3>
//             <ul>
//             {props.data.iter().map(|n|html!{<li>{&n.item}</li>}).collect::<Vec<Html>>()}
//         </ul>
//             </div>
//     }
// }
