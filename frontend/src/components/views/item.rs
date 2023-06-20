use crate::prelude::*;
use crate::Route;
use log::info;

use crate::components::blocks::text_input::*;
#[function_component(NewItem)]
pub fn new_item() -> Html {
    // let data = use_state(|| None);

    // {
    //     let data = data.clone();

    //     use_effect(move || {
    //         if data.is_none() {
    //             spawn_local(async move {
    //                 let resp = Request::get("/api/location/list").send().await.unwrap();
    //                 let result = resp.json::<ServerResponse<Vec<Locations>>>().await.unwrap();

    //                 data.set(Some(result));
    //             });
    //         }
    //     || {}
    //     });
    // }
    let data = use_state(|| None);

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

                show_new_item(&data)
            }
        },
    }
}

fn show_new_item(data: &Vec<Locations>) -> Html {
    let state = use_state(|| None);
    let state_clone = state.clone();

    let submit = Callback::from(move |data: String| {
        let state_clone = state_clone.clone();

        spawn_local(async move {
            let body = serde_json::to_string(&data).unwrap();
            let resp = Request::post("/api/item/add")
                .body(body)
                .header("content-type", "application/json")
                .send()
                .await
                .unwrap();

            let result = resp.json::<ServerResponse<bool>>().await.unwrap();
            state_clone.set(Some(result));
        });
    });

    let msg = match state.as_ref() {
        None => None,
        Some(data) => match data.error {
            false => Some(html! {<div>{"success?"}</div>}),
            true => Some(html! {<div class={"notification"}>{data.message.clone()}</div>}),
        },
    };

    html! {<>
        {msg}
        <AddNewItem onsubmit={submit} />
            </>
    }
}

#[derive(Properties, PartialEq)]
struct Props {
    onsubmit: Callback<String>,
}

#[function_component(AddNewItem)]
fn add_new_item(props: &Props) -> Html {
    let state = use_state(|| None);
    let cloned = state.clone();

    let item_changed = Callback::from(move |e: String| cloned.set(e));

    let form_onsubmit = props.onsubmit.clone();
    let cloned = state.clone();
    let onsubmit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        let data = (*cloned).clone();
        form_onsubmit.emit(data);
    });

    html! {
        <div><h3 class="title">{"Add new item"}</h3>
            <div class="control">
            {data.iter().map(|n| html!{
                                          <>
                                              <label class="radio">
                                              <input type="radio" name={n.name.clone()}/>{n.name.clone()}
                                              </label>
                                          </>
                                      }).collect::<Vec<Html>>()}
            </div>
            <div class="control">
                <TextInput class="input" placeholder="location name" name="item" onchange={item_changed} />
                </div>
        </div>
    }
}
