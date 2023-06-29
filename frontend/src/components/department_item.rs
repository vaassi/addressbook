use stylist::{style, Style};
use web_sys::MouseEvent;
use yew::prelude::*;

use crate::services::api::Department;

#[derive(PartialEq, Properties)]
pub struct DepartmentItemProps {
    pub department: Department,
    pub active: bool,
    pub onclick: Callback<u32>,
}

#[function_component]
pub fn DepartmentItem(props: &DepartmentItemProps) -> Html {
    let onclick = {
        let id = props.department.id;
        let cb = props.onclick.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            cb.emit(id);
        })
    };

    let style = get_style().unwrap();

    html! {
        if props.active {
            <li key={props.department.id} class={classes!("nav-item", "active", style.get_class_name().to_string())}>
                <a {onclick} class="nav-link" href="#">
                    <i class="bi bi-bookmark-check pe-1"></i>{&props.department.name}<span class="badge float-end">{&props.department.count}</span>
                </a>
            </li>
        } else {
            <li key={props.department.id} class={classes!("nav-item", style.get_class_name().to_string())}>
                <a {onclick} class="nav-link" href="#">
                    <i class="bi bi-bookmark pe-1"></i>{&props.department.name}<span class="badge float-end">{&props.department.count}</span>
                </a>
            </li>
        }
    }
}

fn get_style() -> stylist::Result<Style> {
    style!(
        r#"
            a {
                color: var(--bs-gray);
                font-weight: 400;
                font-size: 13px;
                border-radius: 0 !important;
                margin-right: 10px;
                margin-left: 10px;
                padding-right: 0;
            }

            a:hover, a:focus {
                color: var(--bs-dark);
            }

            .badge {
                background: none;
                font-weight: 500;
                color: var(--bs-gray-dark);
                font-size: 12px;
            }

            &.active .badge {
                background: none;
                color: #fff;
            }

            &.active a, &.active a:hover, &.active a:focus {
                color: #fff !important;
                background-color: #2ECC71 !important;
            }
        "#
    )
}
