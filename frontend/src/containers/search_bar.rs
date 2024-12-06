use stylist::{style, Style};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::components::{Button, SearchInput};
use crate::router::Route;
use crate::store::reducers::{page_reducer, search_reducer};
use crate::store::LocalStore;

#[derive(PartialEq, Properties)]
pub struct SearchBarProps {}

#[function_component]
pub fn SearchBar(_props: &SearchBarProps) -> Html {
    let (_, dispatch) = use_store::<LocalStore>();
    let navigator = use_navigator().unwrap();

    let input_ref = NodeRef::default();

    let onchange = {
        let dispatch = dispatch.clone();
        let navigator = navigator.clone();

        Callback::from(move |value: String| {
            search_reducer(dispatch.clone(), value);
            page_reducer(dispatch.clone(), 0);
            navigator.push(&Route::Home);
        })
    };

    let onclick = {
        let input_ref = input_ref.clone();
        let dispatch = dispatch.clone();

        Callback::from(move |_| {
            let value = input_ref.cast::<HtmlInputElement>().unwrap().value();
            search_reducer(dispatch.clone(), value);
            page_reducer(dispatch.clone(), 0);
            navigator.push(&Route::Home);
        })
    };

    use_effect_with_deps(
        move |_| {
            search_reducer(dispatch.clone(), String::new());
        },
        (),
    );

    let style = get_style().unwrap();

    html! {
        <div class={classes!("card", "bg-light", style.get_class_name().to_string())}>
            <div class="card-body input-group">
                <SearchInput {input_ref} placeholder={"Search people"} {onchange} />
                // <Button><i class="bi bi-x-lg"></i></Button>
                <Button onclick={onclick}><i class="bi bi-search"></i></Button>
            </div>
        </div>
    }
}

fn get_style() -> stylist::Result<Style> {
    style!(
        r#"
            border-radius: 10px;
            border: none;
            margin-bottom: 20px;

            button {
                background-color: #2ECC71 !important;
                border: none;
            }

            button:hover {
                background-color: #009A6B !important;
            }

            button + button {
                border-left: 1px dashed var(--bs-white);
                z-index: 6 !important;
            }

            button + button:hover {
                border-left: 1px dashed var(--bs-white);
            }

            .tt-query {
                box-shadow: inset 0 1px 1px rgba(0, 0, 0, 0.075);
            }

            .tt-hint {
              color: #999
            }

            .tt-menu {
              width: 422px;
              margin: 5px 0;
              padding: 5px 0;
              background-color: #fff;
              border: 1px solid #ccc;
              border: 1px solid rgba(0, 0, 0, 0.2);
              border-radius: 5px;
              box-shadow: 0 5px 10px rgba(0,0,0,.2);
            }

            .tt-suggestion {
              padding: 3px 20px;
              font-size: 18px;
              line-height: 24px;
            }

            .tt-suggestion:hover {
              cursor: pointer;
              color: #fff;
              background-color: #2ECC71;
            }

            .tt-suggestion.tt-cursor {
              color: #fff;
              background-color: #2ECC71;
            }

            .tt-suggestion p {
              margin: 0;
            }

            .input-group > .twitter-typeahead {
                position: relative;
                -ms-flex: 1 1 auto;
                -webkit-box-flex: 1;
                flex: 1 1 auto;
                width: 1%;
                margin-bottom: 0;
            }

            .input-group > .twitter-typeahead:not(:last-child) {
                border-top-right-radius: 0;
                border-bottom-right-radius: 0;
            }

            .input-group > .twitter-typeahead > .tt-input {
                 border-top-right-radius: 0;
                 border-bottom-right-radius: 0;
            }

            .form-control.tt-input:focus {
                z-index: 3;
            }
        "#
    )
}
