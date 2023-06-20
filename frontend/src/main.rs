mod components;
mod views;

mod prelude {
    pub use gloo_net::http::Request;
    pub use std::collections::HashMap;
    pub use wasm_bindgen_futures::spawn_local;
    pub use yew::prelude::*;
    pub use yew_router::prelude::*;

    pub use common::*;
}

use crate::components::views::item::*;
use crate::components::views::locations::*;
use crate::components::views::navbar::*;
use crate::components::views::overview::*;

use prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,

    #[not_found]
    #[at("/404")]
    NotFound,

    #[at("/locations")]
    Locations,

    #[at("/locations/delete/:id")]
    LocationDelete { id: i32 },

    #[at("/locations/new")]
    AddLocation,

    #[at("/item/new")]
    NewItem,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Overview />},
        Route::NotFound => html! {<div>{"not found"}</div>},
        Route::Locations => html! {<LocationsList />},
        Route::LocationDelete { id } => html! {
            <LocationDelete id={id} />
        },
        Route::AddLocation => html! {<AddLocation />},
        Route::NewItem => html! {<NewItem />},
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <NavBar />
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
