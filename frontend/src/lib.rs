use stylist::css;
use stylist::yew::Global;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::containers::{DepartmentList, FavoriteList, SearchBar};
use crate::router::{switch, Route};

mod components;
mod containers;
mod error;
mod pages;
mod router;
mod services;
mod store;

#[function_component(App)]
pub fn app() -> Html {
    let style = css!(
        r#"
            body {
                margin-top: 20px;
                margin-bottom: 20px;
                background: #eee;
                font-family: "Helvetica Neue", Helvetica, Arial, sans-serif;
            }
        "#
    );

    html! {
        <BrowserRouter>
            <Global css={style} />
            <div class="container">
                <div class="row">
                    <div class="col-md-3">
                        <DepartmentList />
                        <FavoriteList />
                    </div>
                    <div class="col-md-9">
                        <h3 class="mb-3"><i class="bi bi-journal-text pe-1"></i>{"AddressBook"}</h3>
                        <SearchBar />
                        <Switch<Route> render={switch} />
                    </div>
                </div>
            </div>
        </BrowserRouter>
    }
}
