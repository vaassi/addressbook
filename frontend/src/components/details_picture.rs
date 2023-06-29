use stylist::style;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct DetailsPictureProps {
    pub image: AttrValue,
}

#[function_component]
pub fn DetailsPicture(props: &DetailsPictureProps) -> Html {
    let style = style!(
        r#"
            img {
                height: 10rem;
            }

            &.card {
                border-top-left-radius: 0;
                border-top-right-radius: 0;
                border: none;
            }
        "#
    )
    .unwrap();

    html! {
        <div class={classes!("card", "mb-4", "mb-xl-0", style.get_class_name().to_string())}>
            <div class="card-header"><i class="bi bi-person-bounding-box pe-1"></i>{"Picture"}</div>
            <div class="card-body text-center">
                <img class="rounded-circle mb-2" src={&props.image} alt="photo" />
            </div>
        </div>
    }
}
