use dotenvy_macro::dotenv;
use gloo_net::http::Request;
use serde::Deserialize;

use crate::error::Result;
use crate::pages::Deps;
use crate::store::Contact;

pub const API_ROOT: &str = dotenv!("API_ROOT");

#[derive(Deserialize, PartialEq, Clone)]
pub struct Department {
    pub id: u32,
    pub name: String,
    pub count: u32,
}

pub async fn get_departments() -> Result<Vec<Department>> {
    let departments = Request::get(&format!("{}/api/departments", API_ROOT))
        .header("Content-Type", "application/json")
        .send()
        .await?
        .json::<Vec<Department>>()
        .await?;

    Ok(departments)
}

#[derive(Deserialize)]
pub struct ContactListResponse {
    pub total: u32,
    pub page_size: u32,
    pub page: u32,
    pub data: Vec<Contact>,
}

pub async fn get_contacts(deps: &Deps) -> Result<ContactListResponse> {
    let mut contacts = Request::get(&format!("{}/api/contacts", API_ROOT))
        .query([("page", deps.page.to_string())]);

    if !deps.name.is_empty() {
        contacts = contacts.query([("name", &deps.name)]);
    }

    if deps.dep_id > 0 {
        contacts = contacts.query([("dep_id", deps.dep_id.to_string())]);
    }

    let contacts = contacts
        .header("Content-Type", "application/json")
        .send()
        .await?
        .json::<ContactListResponse>()
        .await?;

    Ok(contacts)
}

pub async fn get_contact_by_id(id: u32) -> Result<Contact> {
    let contact = Request::get(&format!("{}/api/contact/{}", API_ROOT, id))
        .header("Content-Type", "application/json")
        .send()
        .await?
        .json::<Contact>()
        .await?;

    Ok(contact)
}
