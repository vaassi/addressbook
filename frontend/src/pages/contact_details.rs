use gloo_console::log;
use stylist::style;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{Button, DetailsInfo, DetailsPicture};
use crate::router::Route;
use crate::services::api::get_contact_by_id;
use crate::store::Contact;

#[derive(PartialEq, Properties)]
pub struct ContactDetailsProps {
    pub id: u32,
}

#[function_component]
pub fn ContactDetails(props: &ContactDetailsProps) -> Html {
    let contact_state = use_state(Contact::default);
    let navigator = use_navigator().unwrap();
    let location = use_location().unwrap();

    let onclick = {
        let navigator = navigator.clone();

        Callback::from(move |_| {
            navigator.push(&Route::Home);
        })
    };

    let id = props.id;
    let state = contact_state.clone();
    let path = location.path().to_string();
    use_effect_with_deps(
        move |_| {
            spawn_local(async move {
                match get_contact_by_id(id).await {
                    Ok(contact) => {
                        state.set(contact);
                    }
                    Err(e) => {
                        log!(e.to_string());
                        navigator.push(&Route::NotFound)
                    }
                }
            })
        },
        path,
    );

    let h3_style = style!(
        r#"
            font-size: 24px;
            margin-top: 20px;
        "#
    )
    .unwrap();

    let style = style!(
        r#"
            button {
                background-color: #2ECC71 !important;
                border: none;
                border-radius: 3px;
            }

            button:hover {
                background-color: #009A6B !important;
            }
        "#
    )
    .unwrap();

    html! {
        <>
            <h3 class={h3_style}>{"Contact Details"}</h3>
            <div class="row">
                <div class="col-xl-4">
                    <DetailsPicture image={(*contact_state).get_image()} />
                </div>
                <div class="col-xl-8">
                    <DetailsInfo contact={(*contact_state).clone()} />
                </div>
            </div>
            <div class={classes!("float-end", style.get_class_name().to_string())}>
                <Button title="Back" {onclick}  />
            </div>
        </>
    }
}
