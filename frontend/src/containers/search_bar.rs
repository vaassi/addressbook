use stylist::style;
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
    let input_state = use_state(String::default);
    let (_, dispatch) = use_store::<LocalStore>();
    let navigator = use_navigator().unwrap();

    let onchange = {
        let state = input_state.clone();
        let dispatch = dispatch.clone();
        let navigator = navigator.clone();

        Callback::from(move |value: String| {
            state.set(value.clone());
            if value.len() > 2 {
                search_reducer(dispatch.clone(), value);
            } else {
                search_reducer(dispatch.clone(), String::new());
            }
            page_reducer(dispatch.clone(), 0);
            navigator.push(&Route::Home);
        })
    };

    let onclick_clean = {
        let cb = onchange.clone();

        Callback::from(move |_| {
            cb.emit(String::new());
        })
    };

    let onclick = Callback::from(move |_| {
        navigator.push(&Route::Home);
    });

    let state = input_state.clone();
    use_effect_with_deps(
        move |_| {
            search_reducer(dispatch.clone(), (*state).clone());
        },
        (),
    );

    let style = style!(
        r#"
            border-radius: 0;
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
        "#
    )
    .unwrap();

    html! {
        <div class={classes!("card", "bg-light", style.get_class_name().to_string())}>
            <div class="card-body input-group">
                <SearchInput placeholder={"Search people"} value={(*input_state).clone()} {onchange} />
                <Button onclick={onclick_clean}><i class="bi bi-x-lg"></i></Button>
                <Button onclick={onclick}><i class="bi bi-search"></i></Button>
            </div>
        </div>
    }
}
