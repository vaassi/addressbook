use stylist::style;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct PageItemProps {
    pub page: u32,
    pub active: bool,
    pub onclick: Callback<u32>,
}

#[function_component]
pub fn PageItem(props: &PageItemProps) -> Html {
    let onclick = {
        let page = props.page - 1;
        let cb = props.onclick.clone();

        Callback::from(move |_| {
            cb.emit(page);
        })
    };

    let style = style!(
        r#"
            border-bottom-left-radius: 0 !important;
            border-bottom-right-radius: 0 !important;
            color: var(--bs-secondary-color);

            :hover {
                color: var(--bs-body-color);
                cursor: pointer;
            }

            &.active {
                background-color: #2ECC71;
                border-color: #2ECC71;
            }

            &.active:hover {
                color: var(--bs-pagination-active-color);
                cursor: pointer;
            }
        "#
    )
    .unwrap();

    html! {
        <li key={props.page} class="page-item" {onclick}><a class={classes!("page-link", props.active.then_some("active"), style.get_class_name().to_string())} href="#">{props.page}</a></li>
    }
}
