use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::containers::{ContactList, PageList};
use crate::services::api::get_contacts;
use crate::store::reducers::page_reducer;
use crate::store::LocalStore;

#[derive(PartialEq, Clone)]
pub struct Deps {
    pub name: String,
    pub dep_id: u32,
    pub page: u32,
}

#[derive(PartialEq, Properties)]
pub struct HomeProps {}

#[function_component]
pub fn Home(_props: &HomeProps) -> Html {
    let contacts_state = use_state(Vec::new);
    let total_state = use_state(|| 0);
    let (store, dispatch) = use_store::<LocalStore>();

    let state = contacts_state.clone();
    let total = total_state.clone();
    let deps = Deps {
        name: store.search.clone(),
        dep_id: store.dep_id,
        page: store.page,
    };
    let query_params = deps.clone();
    use_effect_with_deps(
        move |_| {
            spawn_local(async move {
                let contacts = get_contacts(&query_params).await.unwrap();
                state.set(contacts.data);
                total.set(contacts.total);
                page_reducer(dispatch, contacts.page);
            })
        },
        deps,
    );

    html! {
        <>
            <div class="row">
                <div class="col">
                    <h3 class="pt-2">{"All Contacts"}</h3>
                </div>
                <div class="col">
                    <PageList total={*total_state} />
                </div>
            </div>
            <ContactList contacts={(*contacts_state).clone()} />
        </>
    }
}
