use stylist::style;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct PageItemProps {
    pub page: PageLimit,
    pub onclick: Callback<PageLimit>,
    pub disabled: bool,
}

#[function_component]
pub fn PageItemLimit(props: &PageItemProps) -> Html {
    let onclick = {
        let page = props.page.clone();
        let cb = props.onclick.clone();

        Callback::from(move |_| {
            cb.emit(page.clone());
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
        "#
    )
    .unwrap();

    html! {
        <li class={classes!("page-item", props.disabled.then_some("disabled"))} {onclick}><a class={classes!("page-link", style.get_class_name().to_string())} href="#">{props.page.to_string()}</a></li>
    }
}

#[derive(PartialEq, Clone)]
pub enum PageLimit {
    Prev,
    Next,
}

impl ToString for PageLimit {
    fn to_string(&self) -> String {
        match self {
            PageLimit::Prev => "\u{00ab}".to_string(),
            PageLimit::Next => "\u{00bb}".to_string(),
        }
    }
}
