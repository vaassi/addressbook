use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct SearchInputProps {
    pub value: AttrValue,
    pub placeholder: AttrValue,
    pub onchange: Callback<String>,
}

#[function_component]
pub fn SearchInput(props: &SearchInputProps) -> Html {
    let onchange = {
        let cb = props.onchange.clone();

        Callback::from(move |e: Event| {
            let value = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            cb.emit(value);
        })
    };

    html! {
        <input class="form-control" type="text" value={&props.value} placeholder={&props.placeholder} {onchange} />
    }
}
