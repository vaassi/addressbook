use stylist::style;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct TextInputProps {
    pub label: AttrValue,
    pub value: AttrValue,
}

#[function_component]
pub fn TextInput(props: &TextInputProps) -> Html {
    let id = format!(
        "input-{}",
        props.label.to_string().to_lowercase().replace(' ', "-")
    );

    let style = style!(
        r#"
            &.form-control {
                padding: 0.875rem 1.125rem;
                font-size: 0.875rem;
                line-height: 1;
                color: #69707a;
                border: 1px solid #c5ccd6;
                border-radius: 3px;
            }
        "#
    )
    .unwrap();

    html! {
        <>
            <label class="small mb-1" for={id.clone()}>{&props.label}</label>
            <input class={classes!("form-control", style.get_class_name().to_string())} type="text" {id} value={&props.value} readonly={true} />
        </>
    }
}
