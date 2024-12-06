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
            <div class="card-header"><i class="bi bi-person-vcard pe-1"></i>{"Information"}</div>
            <div class="card-body">
                <div class="row gx-3 mb-3">
                    <div class="col-md-6">
                        <TextInput label="First name" value={name[0].clone()} />
                    </div>
                    <div class="col-md-6">
                        <TextInput label="Last name" value={name[1].clone()} />
                    </div>
                </div>
                <div class="row gx-3 mb-3">
                    <div class="col-md-6">
                        <TextInput label={"Job title"} value={props.contact.job_title.clone().unwrap_or_default()} />
                    </div>
                    <div class="col-md-6">
                        <TextInput label={"Birthday"} value={props.contact.birthdate.clone().unwrap_or_default()} />
                    </div>
                </div>
                <div class="mb-3">
                    <TextInput label={"Email address"} value={props.contact.mail.clone().unwrap_or_default()} />
                </div>
                <div class="row gx-3 mb-3">
                    <div class="col-md-6">
                        <TextInput label={"Phone number"} value={props.contact.telephone_number.clone().unwrap_or_default()} />
                    </div>
                    <div class="col-md-6">
                        <TextInput label={"Mobile number"} value={props.contact.mobile.clone().unwrap_or_default()} />
                    </div>
                </div>
                <div class="row gx-3 mb-3">
                    <div class="col-md-6">
                        <TextInput label={"Organization name"} value={props.contact.company.clone().unwrap_or_default()} />
                    </div>
                    <div class="col-md-6">
                        <TextInput label={"Employee number"} value={props.contact.employee_number.clone().unwrap_or_default()} />
                    </div>
                </div>
                <div class="mb-3">
                    <TextInput label={"Location"} value={props.contact.get_location_full()} />
                </div>
            </div>
        </div>
    }
}
