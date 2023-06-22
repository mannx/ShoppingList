use crate::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};

use crate::components::blocks::text_input::*;

struct StateData {
    pub locations: Vec<Locations>,
    pub selected_location: i32, // id of location we have selected, defaults to locations[0].id
}

impl StateData {
    pub fn new() -> Self {
        Self {
            locations: Vec::<Locations>::new(),
            selected_location: 0,
        }
    }
}

#[function_component(NewItem)]
pub fn new_item() -> Html {
    let data = use_state(|| None);
    // let data = use_state(|| StateData::new());
    let form_state = use_state(|| String::from(""));

    {
        let data = data.clone();

        use_effect(move || {
            if data.is_none() {
                // have we loaded yet?
                spawn_local(async move {
                    let resp = Request::get("/api/location/list").send().await.unwrap();
                    let result = resp.json::<ServerResponse<Vec<Locations>>>().await.unwrap();

                    data.set(Some(result));
                });
            }
            || {}
        });
    }

    match data.as_ref() {
        None => html! {<div>{"Loading..."}</div>},
        Some(data) => match data.error {
            true => {
                html! {<div>{"Error occured: "}{data.message.clone().unwrap_or(String::from("No error message provided"))}</div>}
            }
            false => {
                let data = data.data.clone().unwrap();

                show_new_item(&data, form_state)
            }
        },
    }
}

fn show_new_item(data: &Vec<Locations>, state: UseStateHandle<String>) -> Html {
    let item_changed = Callback::from(move |e: String| {
        state.set(e);
    });

    let on_select = Callback::from(move |e: Event| {
        log::debug!("select changed!");

        let target: EventTarget = e.target().expect("Require target onchange");

        let n = target.unchecked_into::<HtmlInputElement>().value();
        log::debug!("{:?}", n);
    });

    html! {
        <form>
            <h3 class="title">{"Add New Item"}</h3>
            <div class="field">
                <div class="control">
                    <select id="store" name="store" onchange={on_select}>
                    {data.iter().map(|n| html!{
                      <>
                          // <input type="radio" id={n.id.to_string()} name={"store"} onchange={&radio_changed}/>{n.name.clone()}
                          <option value={n.id.to_string()}>{n.name.clone()}</option>
                      </>
                  }).collect::<Vec<Html>>()}
                    </select>
                </div>
            </div>
            <div class="field">
                <div class="control">
                    <TextInput name="name" onchange={item_changed} />
                </div>
            </div>

            <div class="field">
                <div class="control">
                    <button class="button is-link" type="submit">{"Add"}</button>
                </div>
            </div>
        </form>
    }
}
// fn show_new_item(data: &Vec<Locations>) -> Html {
//     let state = use_state(|| None);
//     let state_clone = state.clone();

//     let submit = Callback::from(move |data: String| {
//         let state_clone = state_clone.clone();

//         spawn_local(async move {
//             let body = serde_json::to_string(&data).unwrap();
//             let resp = Request::post("/api/item/add")
//                 .body(body)
//                 .header("content-type", "application/json")
//                 .send()
//                 .await
//                 .unwrap();

//             let result = resp.json::<ServerResponse<bool>>().await.unwrap();
//             state_clone.set(Some(result));
//         });
//     });

//     let msg = match state.as_ref() {
//         None => None,
//         Some(data) => match data.error {
//             false => Some(html! {<div>{"success?"}</div>}),
//             true => Some(html! {<div class={"notification"}>{data.message.clone()}</div>}),
//         },
//     };

//     html! {{msg}}
// }

// #[derive(Properties, PartialEq)]
// struct Props {
//     onsubmit: Callback<String>,
// }

// #[function_component(AddNewItem)]
// fn add_new_item(props: &Props) -> Html {
//     let state = use_state(|| None);
//     let cloned = state.clone();

//     let item_changed = Callback::from(move |e: String| cloned.set(e));

//     let form_onsubmit = props.onsubmit.clone();
//     let cloned = state.clone();
//     let onsubmit = Callback::from(move |e: SubmitEvent| {
//         e.prevent_default();control
//         let data = (*cloned).clone();
//         form_onsubmit.emit(data);
//     });

//     html! {
//         <div><h3 class="title">{"Add new item"}</h3>
//             <div class="control">
//             {data.iter().map(|n| html!{
//                                           <>
//                                               <label class="radio">
//                                               <input type="radio" name={n.name.clone()}/>{n.name.clone()}
//                                               </label>
//                                           </>
//                                       }).collect::<Vec<Html>>()}
//             </div>
//             <div class="control">
//                 <TextInput class="input" placeholder="location name" name="item" onchange={item_changed} />
//                 </div>
//         </div>
//     }
// }
