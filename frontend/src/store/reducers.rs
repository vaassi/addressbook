use yewdux::dispatch::Dispatch;

use crate::store::{Contact, LocalStore};

pub fn department_reducer(dispatch: Dispatch<LocalStore>, id: u32) {
    dispatch.reduce_mut(|store| store.dep_id = id)
}

pub fn page_reducer(dispatch: Dispatch<LocalStore>, page: u32) {
    dispatch.reduce_mut(|store| {
        store.page = page;
    })
}

pub fn favorite_add_reducer(dispatch: Dispatch<LocalStore>, contact: Contact) {
    dispatch.reduce_mut(|store| {
        if let Some(index) = store
            .favorites
            .iter()
            .position(|Contact { id, .. }| *id == contact.id)
        {
            store.favorites.remove(index);
        } else {
            store.favorites.push(contact);
        }
    })
}

pub fn favorite_del_reducer(dispatch: Dispatch<LocalStore>, c_id: u32) {
    dispatch.reduce_mut(|store| {
        store.favorites.retain(|Contact { id, .. }| *id != c_id);
    })
}

pub fn search_reducer(dispatch: Dispatch<LocalStore>, value: String) {
    dispatch.reduce_mut(|store| store.search = value)
}
