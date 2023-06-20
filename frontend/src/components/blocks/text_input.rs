use crate::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq)]
pub struct TextInputProps {
    pub name: String,
    pub onchange: Callback<String>,
    pub placeholder: String,
    pub class: String,
}

#[function_component(TextInput)]
pub fn text_input(props: &TextInputProps) -> Html {
    let handler = props.onchange.clone();

    let onchange = Callback::from(move |e: Event| {
        let input = e
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        handler.emit(input);
    });

    html! {
        <input class={props.class.clone()} type="text" onchange={onchange} name={props.name.clone()} />
    }
}
