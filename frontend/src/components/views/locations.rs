use crate::prelude::*;
use crate::Route;
use log::info;

use crate::components::blocks::text_input::*;

#[function_component(LocationsList)]
pub fn location_list() -> Html {
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

                show_location_list(&data)
            }
        },
    }
}

#[derive(Properties, PartialEq)]
pub struct LocationProps {
    pub id: i32,
}

// <LocationDelete> is used to send a delete request to the backend and display its response
#[function_component(LocationDelete)]
pub fn location_delete(props: &LocationProps) -> Html {
    info!("Location Deleting id: {}", props.id);

    html! {
        <Redirect<Route> to={Route::Locations} />
    }
}

fn show_location_list(data: &Vec<Locations>) -> Html {
    html! {
        <div><h3 class="title">{"Location List Manangement"}</h3>
            <div class="field is-grouped">
                <p class="control">
                    <Link<Route> to={Route::AddLocation}><button class="button is-link">{"New"}</button></Link<Route>>
                </p>
            </div>

            <ul>
        {data.iter().map(|n| html!{
                                      <li>
                                          <Link<Route> to={Route::LocationDelete{id: n.id}}>
                                          <button class="delete"></button></Link<Route>>
                                      {n.name.clone()}</li>
                                  }).collect::<Vec<Html>>()}
        </ul></div>
    }
}

#[derive(Properties, PartialEq)]
struct Props {
    onsubmit: Callback<Locations>,
}

#[function_component(AddLocation)]
pub fn add_location() -> Html {
    let state = use_state(|| None);
    let state_clone = state.clone();

    let submit = Callback::from(move |data: Locations| {
        let state_clone = state_clone.clone();

        spawn_local(async move {
            let body = serde_json::to_string(&data).unwrap();
            let resp = Request::post("/api/location/add")
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
        <AddLocationForm onsubmit={submit} />
            </>
    }
}

#[function_component(AddLocationForm)]
fn location_add(props: &Props) -> Html {
    let state = use_state(|| Locations::default());
    let cloned = state.clone();

    let loc_changed = Callback::from(move |e: String| {
        cloned.set(Locations {
            id: 0, // we will ignore this in backend
            name: e,
        });
    });

    let form_onsubmit = props.onsubmit.clone();
    let cloned = state.clone();
    let onsubmit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        let data = (*cloned).clone();
        form_onsubmit.emit(data);
    });

    html! {
        <form onsubmit={onsubmit}>
            <h3 class="title">{"Add New Location"}</h3>
            <div class="field">
                <label class="label">{"Location Name"}</label>
                <div class="control">
                    // <input class="input" type="text" placeholder="location name" name="name" onchange={loc_changed}/>
                    <TextInput class="input" placeholder="location name" name="name" onchange={loc_changed} />
                </div>
            </div>
            <div class="field">
                <div class="control">
                    <button class="button is-link" type="submit">{"Submit"}</button>
                </div>
                <div class="control">
                    <Link<Route> to={Route::Locations}>
                    <button class="button is-link is-light">{"Cancel"}</button>
                    </Link<Route>>
                </div>
            </div>
        </form>
    }
}
