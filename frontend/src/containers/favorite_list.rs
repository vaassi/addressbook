use stylist::style;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::components::FavoriteItem;
use crate::store::reducers::favorite_del_reducer;
use crate::store::LocalStore;

#[derive(PartialEq, Properties)]
pub struct FavoriteListProps {}

#[function_component]
pub fn FavoriteList(_props: &FavoriteListProps) -> Html {
    let (store, dispatch) = use_store::<LocalStore>();

    let onclick = Callback::from(move |id| {
        favorite_del_reducer(dispatch.clone(), id);
    });

    let style = style!(
        r#"
            font-size: 14px;
            text-transform: uppercase;
            font-weight: bold;
        "#
    )
    .unwrap();

    html! {
        <>
            <h5 class={style}><i class="bi bi-star pe-1"></i>{"My Favorites"}</h5>
            <div class="list-group mb-3">
                {store.favorites.iter().map(|contact| {
                    html! { <FavoriteItem contact={contact.clone()} onclick={onclick.clone()} /> }
                }).collect::<Html>()}
            </div>
        </>
    }
}
