use stylist::{css, style};
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

    let style_h3 = style!(
        r#"
            a {
                text-decoration: none;
                color: var(--bs-body-color);
            }
        "#
    ).unwrap();

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
                        <h3 class={classes!("mb-3", style_h3.get_class_name().to_string())}>
                            <i class="bi bi-journal-text pe-1"></i>
                            <Link<Route> to={Route::Home}>{"AddressBook"}</Link<Route>>
                        </h3>
                        <SearchBar />
                        <Switch<Route> render={switch} />
                    </div>
                </div>
            </div>
        </BrowserRouter>
    }
}
