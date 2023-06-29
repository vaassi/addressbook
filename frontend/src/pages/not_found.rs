use stylist::style;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[derive(PartialEq, Properties)]
pub struct NotFoundProps {}

#[function_component]
pub fn NotFound(_props: &NotFoundProps) -> Html {
    let style = style!(
        r#"
            margin-top: 20px;
        "#
    )
    .unwrap();

    html! {
         <div class={classes!("row", "justify-content-center", style.get_class_name().to_string())}>
            <div class="col-md-12 text-center">
                <span class="display-1 d-block">{"404"}</span>
                <div class="mb-4 lead">{"The page you are looking for was not found."}</div>
                <Link<Route> to={Route::Home} classes="btn btn-link">{"Back to Home"}</Link<Route>>
            </div>
        </div>
    }
}
