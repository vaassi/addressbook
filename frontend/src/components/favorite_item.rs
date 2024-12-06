use stylist::{style, Style};
use web_sys::MouseEvent;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;
use crate::store::Contact;

#[derive(PartialEq, Properties)]
pub struct FavoriteItemProps {
    pub contact: Contact,
    pub onclick: Callback<u32>,
}

#[function_component]
pub fn FavoriteItem(props: &FavoriteItemProps) -> Html {
    let onclick = {
        let id = props.contact.id;
        let cb = props.onclick.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();

            cb.emit(id)
        })
    };

    let style = get_style().unwrap();

    html! {
        <Link<Route> to={Route::ContactDetails { id: props.contact.id }} key={props.contact.id} classes={classes!("list-group-item", "list-group-item-action", style.get_class_name().to_string())}>
            <div class="d-flex">
                <div class="flex-shrink-0">
                    <img class="rounded-circle" src={props.contact.get_image()} alt="photo" />
                </div>
                <div class="flex-grow-1 ms-3">
                    <h5>
                        if props.contact.is_birthday_today() {
                        <i class="bi bi-balloon-fill me-1"></i>
                        }
                        {&props.contact.name}
                        <i class="bi bi-x float-end" {onclick}></i>
                    </h5>
                    <small class="text-body-secondary text-break">{&props.contact.get_subtitle()}</small>
                </div>
            </div>
        </Link<Route>>
    }
}

fn get_style() -> stylist::Result<Style> {
    style!(
        r#"
            transition: all 0.2s ease-out 0s;
            margin: 0;
            border: none;
            border-radius: 0;

            :first-child {
                border-top-left-radius: 0;
                border-top-right-radius: 0;
            }

            img {
                width: 45px;
            }

            h5 {
                font-size: 18px;
                margin-bottom: 0;
                padding-top: 5px;
            }

            h5 i:hover {
                color: var(--bs-red);
            }

            small {
                font-size: 12px;
            }

            h5 > i.bi.bi-balloon-fill {
                color: var(--bs-red);
            }
        "#
    )
}
