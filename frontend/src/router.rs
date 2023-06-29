use yew::{html, Html};
use yew_router::prelude::*;

use crate::pages::{ContactDetails, Home, NotFound};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/contact/:id")]
    ContactDetails { id: u32 },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::ContactDetails { id } => html! { <ContactDetails {id} /> },
        Route::NotFound => html! { <NotFound /> },
    }
}
