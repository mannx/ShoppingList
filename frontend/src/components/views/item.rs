use crate::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};

use crate::components::blocks::text_input::*;

#[derive(Default, Clone)]
struct StateData {
    pub message: Option<ServerResponse<bool>>,
    pub item_name: String,
    pub store_id: i32,
}

#[function_component(NewItemForm)]
pub fn new_item() -> Html {
    let loc_data = use_state(|| None);
    let state = use_state(|| StateData::default());

    {
        let loc_data = loc_data.clone();

        use_effect(move || {
            if loc_data.is_none() {
                // have we loaded yet?
                spawn_local(async move {
                    let resp = Request::get("/api/location/list").send().await.unwrap();
                    let result = resp.json::<ServerResponse<Vec<Locations>>>().await.unwrap();

                    loc_data.set(Some(result));
                });
            }
            || {}
        });
    }

    match loc_data.as_ref() {
        None => return html! {<div>{"Loading..."}</div>},
        Some(data) => match data.error {
            true => return html! {<div>{"Error occurred: "}{data.message.clone().unwrap()}</div>},
            false => (),
        },
    }

    // state contains valid locations if any from here

    let cloned = state.clone();
    let item_changed = Callback::from(move |e: String| {
        cloned.set(StateData {
            item_name: e,
            ..(*cloned).clone()
        });
    });

    let cloned = state.clone();
    let on_select = Callback::from(move |e: Event| {
        let target: EventTarget = e.target().expect("Require target onchange");
        let n = target.unchecked_into::<HtmlInputElement>().value();
        let val = n.parse::<i32>().unwrap();

        cloned.set(StateData {
            store_id: val,
            ..(*cloned).clone()
        });
    });

    let cloned = state.clone();
    let on_submit = Callback::from(move |e: SubmitEvent| {
        log::debug!("Submiting!");

        e.prevent_default();

        let cloned = cloned.clone();

        spawn_local(async move {
            let data = NewItem {
                store_id: cloned.store_id,
                item_name: cloned.item_name.clone(),
            };

            let body = serde_json::to_string(&data).unwrap();
            let resp = Request::post("/api/item/add")
                .body(body)
                .header("content-type", "application/json")
                .send()
                .await
                .unwrap();

            let result = resp.json::<ServerResponse<bool>>().await.unwrap();
            cloned.set(StateData {
                message: Some(result),
                ..(*cloned).clone()
            });
        });
    });

    let locations = (loc_data.as_ref().unwrap()).data.clone().unwrap();

    let msg = match &state.message {
        None => html! {},
        Some(r) => match r.error {
            false => html! {},
            true => html! {
                <div class="notification">
                {"Error occurred: "}{r.message.clone().unwrap()}
                </div>
            },
        },
    };

    html! {
        <form onsubmit={on_submit}>
            <h3 class="title">{"Add New Item"}</h3>
            {msg}
            <div class="field">
                <div class="control">
                    <select id="store" name="store" onchange={on_select}>
                    {locations.iter().map(|n| html!{
                          <option value={n.id.to_string()} >
                          {n.name.clone()}</option>
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
                    <button class="button is-link" type="submit" >{"Add"}</button>
                </div>
            </div>
        </form>
    }
}
