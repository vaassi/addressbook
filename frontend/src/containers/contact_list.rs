use yew::prelude::*;
use yewdux::prelude::*;

use crate::components::ContactItem;
use crate::store::reducers::favorite_add_reducer;
use crate::store::{Contact, LocalStore};

#[derive(PartialEq, Properties)]
pub struct ContactListProps {
    pub contacts: Vec<Contact>,
}

#[function_component]
pub fn ContactList(props: &ContactListProps) -> Html {
    let (store, dispatch) = use_store::<LocalStore>();

    let onclick = Callback::from(move |contact| {
        favorite_add_reducer(dispatch.clone(), contact);
    });

    html! {
        <div class="list-group">
        if !props.contacts.is_empty() {
            {props.contacts.iter().map(|contact| {
                let favorite = store.favorites.iter().any(|Contact { id, .. }| {*id == contact.id});
                html! { <ContactItem contact={contact.clone()} {favorite} onclick={onclick.clone()} /> }
            }).collect::<Html>()}
        } else {
            <p class="text-center mt-5"><i class="bi bi-emoji-frown"></i>{" can't find any contacts..."}</p>
        }
        </div>
    }
}
