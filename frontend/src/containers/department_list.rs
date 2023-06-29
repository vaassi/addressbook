use stylist::style;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::components::DepartmentItem;
use crate::services::api::get_departments;
use crate::store::reducers::{department_reducer, page_reducer};
use crate::store::LocalStore;

#[derive(PartialEq, Properties)]
pub struct DepartmentListProps {}

#[function_component]
pub fn DepartmentList(_props: &DepartmentListProps) -> Html {
    let department_state = use_state(Vec::new);
    let (store, dispatch) = use_store::<LocalStore>();

    let onclick = {
        let dispatch = dispatch.clone();

        Callback::from(move |id| {
            department_reducer(dispatch.clone(), id);
            page_reducer(dispatch.clone(), 0);
        })
    };

    let state = department_state.clone();
    use_effect_with_deps(
        move |_| {
            spawn_local(async move {
                let departments = get_departments().await.unwrap();
                state.set(departments);
                page_reducer(dispatch.clone(), 0);
            })
        },
        (),
    );

    let h5_style = style!(
        r#"
            font-size: 14px;
            text-transform: uppercase;
            font-weight: bold;
        "#
    )
    .unwrap();

    let ul_style = style!(
        r#"
            margin-bottom: 20px;
        "#
    )
    .unwrap();

    html! {
        <>
            <h5 class={h5_style}><i class="bi bi-journal-bookmark pe-1"></i>{"Departments"}</h5>
            <ul class={classes!("nav", "nav-pills", "flex-column", ul_style.get_class_name().to_string())}>
                { (*department_state).iter().map(|department| {
                    let active = store.dep_id == department.id;
                    html! { <DepartmentItem department={department.clone()} {active} onclick={onclick.clone()} /> }
                }).collect::<Html>()}
            </ul>
        </>
    }
}
