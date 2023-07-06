use web_sys::MouseEvent;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ButtonProps {
    #[prop_or_default]
    pub title: AttrValue,
    #[prop_or_default]
    pub children: Children,
    pub onclick: Callback<()>,
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {

    let onclick = {
        let cb = props.onclick.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            cb.emit(());
        })
    };

    html! {
        <button type="button" class="btn btn-success" {onclick}>
            {for props.children.iter()}{&props.title}
        </button>
    }
}
