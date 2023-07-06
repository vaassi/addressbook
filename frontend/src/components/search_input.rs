use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct SearchInputProps {
    pub input_ref: NodeRef,
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
        <input ref={&props.input_ref} class="form-control typeahead" type="text" placeholder={&props.placeholder} {onchange} />
    }
}
