use stylist::{style, Style};
use web_sys::MouseEvent;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;
use crate::store::Contact;

#[derive(PartialEq, Properties)]
pub struct ContactItemProps {
    pub contact: Contact,
    pub favorite: bool,
    pub onclick: Callback<Contact>,
}

#[function_component]
pub fn ContactItem(props: &ContactItemProps) -> Html {
    let onclick = {
        let cb = props.onclick.clone();
        let contact = props.contact.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            let contact = contact.clone();

            cb.emit(contact);
        })
    };

    let style = get_style().unwrap();

    html! {
        <Link<Route> to={Route::ContactDetails { id: props.contact.id }} key={props.contact.id} classes={classes!("list-group-item", style.get_class_name().to_string())}>
            <div class="d-flex">
                <div class="flex-shrink-0">
                    <img class="rounded-circle mt-2" src={props.contact.get_image()} alt={"photo"} />
                </div>
                <div class="flex-grow-1 ms-3">
                    <h4>
                        if props.contact.is_birthday_today() {
                        <i class="bi bi-balloon-fill me-1"></i>
                        }
                        {&props.contact.name}
                        <small class="ps-2">{props.contact.get_location()}</small>
                        <i class={classes!("bi", "bi-star", "float-end", props.favorite.then_some("favorite"))} {onclick}></i>
                    </h4>
                    <li class="text-break">{props.contact.get_subtitle()}</li>
                    <ul class="list-unstyled">
                        <li><i class="bi bi-telephone"></i>{props.contact.telephone_number.clone().unwrap_or_default()}</li>
                        <li><i class="bi bi-phone"></i>{props.contact.mobile.clone().unwrap_or_default()}</li>
                        <li><i class="bi bi-envelope-at"></i>{props.contact.mail.clone().unwrap_or_default()}</li>
                    </ul>
                </div>
            </div>
        </Link<Route>>
    }
}

fn get_style() -> stylist::Result<Style> {
    style!(
        r#"
            border: none;
            margin-top: 10px;
            font-size: 14px;

            :hover,  {
                background-color: var(--bs-tertiary-bg);
            }

            :first-child {
                border-top-left-radius: 0;
                border-top-right-radius: 0;
            }

            img {
                width: 80px;
            }

            h4 {
                font-size: 16px;
                font-weight: 600;
            }

            small {
                margin-left: 5px;
                font-size: 13px;
                font-weight: 400;
                color: #999;
            }

            ul {
                margin-top: 10px;
                margin-bottom: 0;
            }

            li {
                display: inline-block;
                min-width: 200px;
                margin-bottom: 3px;
            }

            li > i.bi, h4 + i.bi {
                font-size: 20px;
                color: var(--bs-gray-600);
                padding-right: 5px;
            }

            h4 > i.bi.bi-star {
                font-size: 20px;
            }

            h4 > i.bi.bi-star.favorite::before {
                content: "\f586";
                color: var(--bs-yellow);
            }

            h4 > i.bi.bi-balloon-fill {
                color: var(--bs-red);
            }
        "#
    )
}
