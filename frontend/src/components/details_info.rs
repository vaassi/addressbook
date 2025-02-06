use stylist::style;
use yew::prelude::*;

use crate::components::TextInput;
use crate::store::Contact;

#[derive(PartialEq, Properties)]
pub struct DetailsInfoProps {
    pub contact: Contact,
}

#[function_component]
pub fn DetailsInfo(props: &DetailsInfoProps) -> Html {
    let mut name = vec!["".to_string(), "".to_string()];
    if !props.contact.name.is_empty() {
        name = props
            .contact
            .name
            .split(' ')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
    }

    let style = style!(
        r#"
            &.card {
                border: none;
            }
        "#
    )
    .unwrap();

    html! {
        <div class={classes!("card", "mb-4", style.get_class_name().to_string())}>
            <div class="card-header"><i class="bi bi-person-vcard pe-1"></i>{"Информация"}</div>
            <div class="card-body">
                <div class="row gx-3 mb-3">
                    <div class="col-md-6">
                        <TextInput label="Имя" value={name[0].clone()} />
                    </div>
                    <div class="col-md-6">
                        <TextInput label="Фамилия" value={name[1].clone()} />
                    </div>
                </div>
                <div class="row gx-3 mb-3">
                    <div class="col-md-12">
                        <TextInput label={"Должность"} value={props.contact.job_title.clone().unwrap_or_default()} />
                    </div>
                </div>
                <div class="row gx-3 mb-3">
                    <div class="col-md-12">
                        <TextInput label={"Email"} value={props.contact.mail.clone().unwrap_or_default()} />
                    </div>
                </div>
                <div class="row gx-3 mb-3">
                    <div class="col-md-6">
                        <TextInput label={"Телефон"} value={props.contact.telephone_number.clone().unwrap_or_default()} />
                    </div>
                    <div class="col-md-6">
                        <TextInput label={"Мобильный"} value={props.contact.mobile.clone().unwrap_or_default()} />
                    </div>
                </div>
            </div>
        </div>
    }
}
