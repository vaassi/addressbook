use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

use crate::services::api::API_ROOT;

pub mod reducers;

#[derive(Serialize, Deserialize, Store, Clone, Default, PartialEq)]
#[store(storage = "local")]
pub struct LocalStore {
    pub favorites: Vec<Contact>,
    pub dep_id: u32,
    pub search: String,
    pub page: u32,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct Contact {
    pub id: u32,
    pub name: String,
    pub photo: bool,
    pub employee_number: Option<String>,
    pub mail: Option<String>,
    pub telephone_number: Option<String>,
    pub mobile: Option<String>,
    pub birthdate: Option<String>,
    pub company: Option<String>,
    pub department: Option<String>,
    pub job_title: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub state: Option<String>,
    pub city: Option<String>,
    pub street_address: Option<String>,
}

impl Contact {
    pub fn get_image(&self) -> String {
        if let Some(id) = &self.employee_number {
            if self.photo {
                format!("{}/assets/{}.jpg", API_ROOT, id)
            } else {
                format!("{}/images/no-photo.png", API_ROOT)
            }
        } else {
            format!("{}/images/no-photo.png", API_ROOT)
        }
    }

    pub fn get_subtitle(&self) -> String {
        if let Some(company) = &self.company {
            if let Some(job_title) = &self.job_title {
                format!("{} at {}", job_title, company)
            } else {
                company.to_string()
            }
        } else if let Some(job_title) = &self.job_title {
            job_title.to_string()
        } else {
            String::new()
        }
    }

    pub fn get_location(&self) -> String {
        format!(
            "{}, {}, {}",
            self.country.clone().unwrap_or_default(),
            self.state.clone().unwrap_or_default(),
            self.city.clone().unwrap_or_default(),
        )
    }

    pub fn get_location_full(&self) -> String {
        format!(
            "{}, {}, {}, {}, {}",
            self.country.clone().unwrap_or_default(),
            self.postal_code.clone().unwrap_or_default(),
            self.state.clone().unwrap_or_default(),
            self.city.clone().unwrap_or_default(),
            self.street_address.clone().unwrap_or_default()
        )
    }

    pub fn is_birthday_today(&self) -> bool {
        if let Some(mut birthday) = self.birthdate.clone() {
            let today = Local::now().date_naive();
            birthday.push_str(&today.format("/%Y").to_string());
            let birthday = NaiveDate::parse_from_str(&birthday, "%d/%m/%Y").unwrap();

            return today == birthday;
        }
        false
    }
}
