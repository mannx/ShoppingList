use crate::prelude::*;

#[derive(Properties, PartialEq)]
struct ShopDataProp {
    location: String,
    data: Vec<ShoppingData>,
}

#[function_component(Overview)]
pub fn overview() -> Html {
    let data = use_state(|| None);

    {
        let data = data.clone();

        use_effect(move || {
            if data.is_none() {
                // have we loaded yet?
                spawn_local(async move {
                    let resp = Request::get("/api/get").send().await.unwrap();
                    let result = resp
                        .json::<ServerResponse<HashMap<String, Vec<ShoppingData>>>>()
                        .await
                        .unwrap();

                    data.set(Some(result));
                });
            }
            || {}
        });
    }

    match data.as_ref() {
        None => html! {
            <div>{"Loading..."}</div>
        },
        Some(data) => match data.error {
            true => {
                html! { <div>{"Error occurred: "}{data.message.clone().unwrap_or(String::from("No error message provided"))}</div> }
            }
            false => {
                let data = data.data.clone().unwrap();

                view_data(&data)
            }
        },
    }
}

fn view_data(data: &HashMap<String, Vec<ShoppingData>>) -> Html {
    html! {<>
    {data.iter().map(|(k,v)| html!{<ShopData location={k.clone()} data={v.clone()} />}).collect::<Vec<Html>>()}
    </>}
}

#[function_component(ShopData)]
fn shop_data(props: &ShopDataProp) -> Html {
    html! {
        <div class="content"><h3 class="subtitle">{&props.location}</h3>
            <ul>
            {props.data.iter().map(|n|html!{<li>{&n.item}</li>}).collect::<Vec<Html>>()}
        </ul>
            </div>
    }
}
