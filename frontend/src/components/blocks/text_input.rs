use crate::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

// #[derive(Properties, PartialEq)]
// pub struct TextInputProps {
//     pub name: String,
//     pub onchange: Callback<String>,
//     pub placeholder: String,
//     pub class: String,
// }

#[derive(Properties, PartialEq)]
pub struct TextInputProps {
    pub name: String,
    pub onchange: Callback<String>,
    pub placeholder: Option<String>,
    pub class: Option<String>,
}

// #[function_component(TextInput)]
// pub fn text_input(props: &TextInputProps) -> Html {
//     let handler = props.onchange.clone();

//     let onchange = Callback::from(move |e: Event| {
//         let input = e
//             .target()
//             .unwrap()
//             .unchecked_into::<HtmlInputElement>()
//             .value();
//         handler.emit(input);
//     });

//     html! {
//         <input class={props.class.clone()} type="text" onchange={onchange} name={props.name.clone()} />
//     }
// }

#[function_component(TextInput)]
pub fn text_input2(props: &TextInputProps) -> Html {
    let handler = props.onchange.clone();

    let onchange = Callback::from(move |e: Event| {
        let input = e
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        handler.emit(input);
    });

    let placeholder = match &props.placeholder {
        Some(n) => n.clone(),
        None => String::from(""),
    };

    let class = props.class.clone().unwrap_or(String::from(""));

    html! {
        <input class={class} placeholder={placeholder} type="text" onchange={onchange} name={props.name.clone()} />
    }
}
