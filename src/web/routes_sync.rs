use std::env;

use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use chrono::{NaiveDate, Utc};
use ldap3::{LdapConnAsync, Scope, SearchEntry};
use sea_orm::{ActiveModelTrait, EntityTrait, ModelTrait, Set};
use serde_json::{json, Value};

use entity::{
    city, company, contact, country, department, job_title, postal_code, state, street_address,
};

use crate::error::Result;
use crate::AppState;

pub fn routes_sync(state: AppState) -> Router {
    Router::new()
        .route("/sync", get(ldap_sync))
        .with_state(state)
}

async fn ldap_sync(State(state): State<AppState>) -> Result<Json<Value>> {
    // get env vars
    let ldap_url = env::var("LDAP_URL").expect("LDAP_URL is not set in .env file");
    let ldap_username = env::var("LDAP_USERNAME").expect("LDAP_USERNAME is not set in .env file");
    let ldap_password = env::var("LDAP_PASSWORD").expect("LDAP_PASSWORD is not set in .env file");
    let ldap_base_dn = env::var("LDAP_BASE_DN").expect("LDAP_BASE_DN is not set in .env file");
    let birthdate_attr =
        env::var("BIRTHDATE_ATTR").expect("BIRTHDATE_ATTR is not set in .env file");
    let birthdate_fmt = env::var("BIRTHDATE_FMT").expect("BIRTHDATE_FMT is not set in .env file");

    // get ldap connection
    let (conn, mut ldap) = LdapConnAsync::new(&ldap_url).await?;
    ldap3::drive!(conn);

    ldap.simple_bind(&ldap_username, &ldap_password)
        .await?
        .success()?;

    // get contacts
    let (rs, _res) = ldap
        .search(
            &ldap_base_dn,
            Scope::Subtree,
            "(&(objectCategory=person)(objectClass=user))",
            vec![
                "name",
                "employeeNumber",
                "mail",
                "telephoneNumber",
                "mobile",
                birthdate_attr.as_str(),
                "company",
                "department",
                "title",
                "c",
                "postalCode",
                "st",
                "l",
                "streetAddress",
            ],
        )
        .await?
        .success()?;

    // ldap.unbind().await?;

    // get companies
    let mut current_companies = company::Entity::find()
        .all(&state.conn)
        .await?
        .iter()
        .map(|m| (m.id, m.name.clone()))
        .collect::<Vec<(i32, String)>>();
    let mut new_companies = vec![];

    // get departments
    let mut current_departments = department::Entity::find()
        .all(&state.conn)
        .await?
        .iter()
        .map(|m| (m.id, m.name.clone()))
        .collect::<Vec<(i32, String)>>();
    let mut new_departments = vec![];

    // get job titles
    let mut current_job_titles = job_title::Entity::find()
        .all(&state.conn)
        .await?
        .iter()
        .map(|m| (m.id, m.name.clone()))
        .collect::<Vec<(i32, String)>>();
    let mut new_job_titles = vec![];

    // get countries
    let mut current_countries = country::Entity::find()
        .all(&state.conn)
        .await?
        .iter()
        .map(|m| (m.id, m.name.clone()))
        .collect::<Vec<(i32, String)>>();
    let mut new_countries = vec![];

    // get postal codes
    let mut current_postal_codes = postal_code::Entity::find()
        .all(&state.conn)
        .await?
        .iter()
        .map(|m| (m.id, m.name.clone()))
        .collect::<Vec<(i32, String)>>();
    let mut new_postal_codes = vec![];

    // get states
    let mut current_states = state::Entity::find()
        .all(&state.conn)
        .await?
        .iter()
        .map(|m| (m.id, m.name.clone()))
        .collect::<Vec<(i32, String)>>();
    let mut new_states = vec![];

    // get cities
    let mut current_cities = city::Entity::find()
        .all(&state.conn)
        .await?
        .iter()
        .map(|m| (m.id, m.name.clone()))
        .collect::<Vec<(i32, String)>>();
    let mut new_cities = vec![];

    // get street addresses
    let mut current_street_addresses = street_address::Entity::find()
        .all(&state.conn)
        .await?
        .iter()
        .map(|m| (m.id, m.name.clone()))
        .collect::<Vec<(i32, String)>>();
    let mut new_street_addresses = vec![];

    for entry in &rs {
        let contact = SearchEntry::construct(entry.clone());

        get_attr_delta(&contact, "company", &current_companies, &mut new_companies);
        get_attr_delta(
            &contact,
            "department",
            &current_departments,
            &mut new_departments,
        );
        get_attr_delta(&contact, "title", &current_job_titles, &mut new_job_titles);
        get_attr_delta(&contact, "c", &current_countries, &mut new_countries);
        get_attr_delta(
            &contact,
            "postalCode",
            &current_postal_codes,
            &mut new_postal_codes,
        );
        get_attr_delta(&contact, "st", &current_states, &mut new_states);
        get_attr_delta(&contact, "l", &current_cities, &mut new_cities);
        get_attr_delta(
            &contact,
            "streetAddress",
            &current_street_addresses,
            &mut new_street_addresses,
        );
    }

    // insert company
    for item in &new_companies {
        let item = company::ActiveModel {
            name: Set(item.to_owned()),
            ..Default::default()
        };

        let item = item.insert(&state.conn).await?;
        println!("Found new company: `{}`", item.name);

        current_companies.push((item.id, item.name));
    }

    // insert department
    for item in &new_departments {
        let item = department::ActiveModel {
            name: Set(item.to_owned()),
            ..Default::default()
        };

        let item = item.insert(&state.conn).await?;
        println!("Found new department: `{}`", item.name);

        current_departments.push((item.id, item.name));
    }

    // insert job title
    for item in &new_job_titles {
        let item = job_title::ActiveModel {
            name: Set(item.to_owned()),
            ..Default::default()
        };

        let item = item.insert(&state.conn).await?;
        println!("Found new job title: `{}`", item.name);

        current_job_titles.push((item.id, item.name));
    }

    // insert country
    for item in &new_countries {
        let item = country::ActiveModel {
            name: Set(item.to_owned()),
            ..Default::default()
        };

        let item = item.insert(&state.conn).await?;
        println!("Found new country: `{}`", item.name);

        current_countries.push((item.id, item.name));
    }

    // insert postal code
    for item in &new_postal_codes {
        let item = postal_code::ActiveModel {
            name: Set(item.to_owned()),
            ..Default::default()
        };

        let item = item.insert(&state.conn).await?;
        println!("Found new postal code: `{}`", item.name);

        current_postal_codes.push((item.id, item.name));
    }

    // insert state
    for item in &new_states {
        let item = state::ActiveModel {
            name: Set(item.to_owned()),
            ..Default::default()
        };

        let item = item.insert(&state.conn).await?;
        println!("Found new state: `{}`", item.name);

        current_states.push((item.id, item.name));
    }

    // insert city
    for item in &new_cities {
        let item = city::ActiveModel {
            name: Set(item.to_owned()),
            ..Default::default()
        };

        let item = item.insert(&state.conn).await?;
        println!("Found new city: `{}`", item.name);

        current_cities.push((item.id, item.name));
    }

    // insert street address
    for item in &new_street_addresses {
        let item = street_address::ActiveModel {
            name: Set(item.to_owned()),
            ..Default::default()
        };

        let item = item.insert(&state.conn).await?;
        println!("Found new street address: `{}`", item.name);

        current_street_addresses.push((item.id, item.name));
    }

    // get all database contacts
    let db_contacts = contact::Entity::find().all(&state.conn).await?;
    let ldap_contacts = rs
        .iter()
        .map(|r| SearchEntry::construct(r.clone()))
        .collect::<Vec<SearchEntry>>();

    // delete contacts that not exist anymore
    for db_contact in &db_contacts {
        if !ldap_contacts.iter().any(|c| c.dn == db_contact.dn) {
            db_contact.clone().delete(&state.conn).await?;
            println!("Deleting contact: `{}`", db_contact.name);
        }
    }

    for ldap_contact in ldap_contacts {
        let name = get_attr_value(&ldap_contact, "name").unwrap();
        let dn = ldap_contact.dn.to_owned();
        let employee_number = get_attr_value(&ldap_contact, "employeeNumber");
        let mail = get_attr_value(&ldap_contact, "mail");
        let telephone_number = get_attr_value(&ldap_contact, "telephoneNumber");
        let mobile = get_attr_value(&ldap_contact, "mobile");
        let photo = if let Some(en) = &employee_number {
            std::path::Path::new(&format!("./static/assets/{}.jpg", en)).exists()
        } else {
            false
        };

        let mut birthdate = get_attr_value(&ldap_contact, birthdate_attr.as_str());
        let mut birthday_fmt = birthdate_fmt.clone();
        if let Some(mut birthday) = birthdate {
            if !birthday_fmt.contains("%Y") {
                birthday_fmt.push_str("/%Y");
                birthday.push_str(&Utc::now().format("/%Y").to_string());
            }
            let birthday = NaiveDate::parse_from_str(&birthday, &birthday_fmt)?;
            birthdate = Some(birthday.format("%d/%m").to_string());
        }

        let company_id = get_attr_id(&current_companies, &ldap_contact, "company");
        let department_id = get_attr_id(&current_departments, &ldap_contact, "department");
        let job_title_id = get_attr_id(&current_job_titles, &ldap_contact, "title");
        let country_id = get_attr_id(&current_countries, &ldap_contact, "c");
        let postal_code_id = get_attr_id(&current_postal_codes, &ldap_contact, "postalCode");
        let state_id = get_attr_id(&current_states, &ldap_contact, "st");
        let city_id = get_attr_id(&current_cities, &ldap_contact, "l");
        let street_address_id =
            get_attr_id(&current_street_addresses, &ldap_contact, "streetAddress");

        if let Some(contact) = db_contacts.iter().find(|c| c.dn == ldap_contact.dn) {
            // update contact
            let mut item: contact::ActiveModel = contact.clone().into();

            if contact.name != name {
                item.name = Set(name.clone());
                println!(
                    "Update contact `{}` name attribute: {} -> {}",
                    contact.name, contact.name, name
                );
            }

            if contact.employee_number != employee_number {
                item.employee_number = Set(employee_number.clone());
                println!(
                    "Update contact `{}` employee_number attribute: {} -> {}",
                    contact.name,
                    contact.employee_number.clone().unwrap_or_default(),
                    employee_number.unwrap_or_default()
                );
            }

            if contact.mail != mail {
                item.mail = Set(mail.clone());
                println!(
                    "Update contact `{}` mail attribute: {} -> {}",
                    contact.name,
                    contact.mail.clone().unwrap_or_default(),
                    mail.unwrap_or_default()
                );
            }

            if contact.telephone_number != telephone_number {
                item.telephone_number = Set(telephone_number.clone());
                println!(
                    "Update contact `{}` telephone_number attribute: {} -> {}",
                    contact.name,
                    contact.telephone_number.clone().unwrap_or_default(),
                    telephone_number.unwrap_or_default()
                );
            }

            if contact.mobile != mobile {
                item.mobile = Set(mobile.clone());
                println!(
                    "Update contact `{}` mobile attribute: {} -> {}",
                    contact.name,
                    contact.mobile.clone().unwrap_or_default(),
                    mobile.unwrap_or_default()
                );
            }

            if contact.photo != photo {
                item.photo = Set(photo);
                println!(
                    "Update contact `{}` photo attribute: {} -> {}",
                    contact.name,
                    contact.photo.clone(),
                    photo
                );
            }

            if contact.birthdate != birthdate {
                item.birthdate = Set(birthdate.clone());
                println!(
                    "Update contact `{}` birthdate attribute: {} -> {}",
                    contact.name,
                    contact.birthdate.clone().unwrap_or_default(),
                    birthdate.unwrap_or_default()
                );
            }

            if contact.company_id != company_id {
                item.company_id = Set(company_id);
                println!(
                    "Update contact `{}` company attribute: {} -> {}",
                    contact.name,
                    contact.company_id.unwrap_or_default(),
                    company_id.unwrap_or_default()
                );
            }

            if contact.department_id != department_id {
                item.department_id = Set(department_id);
                println!(
                    "Update contact `{}` department attribute: {} -> {}",
                    contact.name,
                    contact.department_id.unwrap_or_default(),
                    department_id.unwrap_or_default()
                );
            }

            if contact.job_title_id != job_title_id {
                item.job_title_id = Set(job_title_id);
                println!(
                    "Update contact `{}` job_title attribute: {} -> {}",
                    contact.name,
                    contact.job_title_id.unwrap_or_default(),
                    job_title_id.unwrap_or_default()
                );
            }

            if contact.country_id != country_id {
                item.country_id = Set(country_id);
                println!(
                    "Update contact `{}` country attribute: {} -> {}",
                    contact.name,
                    contact.country_id.unwrap_or_default(),
                    country_id.unwrap_or_default()
                );
            }

            if contact.postal_code_id != postal_code_id {
                item.postal_code_id = Set(postal_code_id);
                println!(
                    "Update contact `{}` postal_code attribute: {} -> {}",
                    contact.name,
                    contact.postal_code_id.unwrap_or_default(),
                    postal_code_id.unwrap_or_default()
                );
            }

            if contact.state_id != state_id {
                item.state_id = Set(state_id);
                println!(
                    "Update contact `{}` state attribute: {} -> {}",
                    contact.name,
                    contact.state_id.unwrap_or_default(),
                    state_id.unwrap_or_default()
                );
            }

            if contact.city_id != city_id {
                item.city_id = Set(city_id);
                println!(
                    "Update contact `{}` city attribute: {} -> {}",
                    contact.name,
                    contact.city_id.unwrap_or_default(),
                    city_id.unwrap_or_default()
                );
            }

            if contact.street_address_id != street_address_id {
                item.street_address_id = Set(street_address_id);
                println!(
                    "Update contact `{}` street_address attribute: {} -> {}",
                    contact.name,
                    contact.street_address_id.unwrap_or_default(),
                    street_address_id.unwrap_or_default()
                );
            }

            item.update(&state.conn).await?;
        } else {
            // insert contact
            let item = contact::ActiveModel {
                name: Set(name),
                dn: Set(dn),
                employee_number: Set(employee_number),
                mail: Set(mail),
                telephone_number: Set(telephone_number),
                mobile: Set(mobile),
                photo: Set(photo),
                birthdate: Set(birthdate),
                company_id: Set(company_id),
                department_id: Set(department_id),
                job_title_id: Set(job_title_id),
                country_id: Set(country_id),
                postal_code_id: Set(postal_code_id),
                state_id: Set(state_id),
                city_id: Set(city_id),
                street_address_id: Set(street_address_id),
                ..Default::default()
            };

            let item = item.insert(&state.conn).await?;
            println!("Found new contact: `{}`", item.name);
        }
    }

    Ok(Json(json!({"status": "success"})))
}

fn get_attr_delta(
    contact: &SearchEntry,
    name: &str,
    current: &[(i32, String)],
    new: &mut Vec<String>,
) {
    if let Some(item) = contact.attrs.get(name) {
        if let Some(item) = item.get(0) {
            if !current.iter().any(|c| c.1 == *item) && !new.contains(item) {
                new.push(item.clone())
            }
        }
    }
}

fn get_attr_id(container: &[(i32, String)], contact: &SearchEntry, name: &str) -> Option<i32> {
    container
        .iter()
        .find(|&c| get_attr_value(contact, name) == Some(c.1.to_owned()))
        .map(|(i, _)| *i)
}

fn get_attr_value(contact: &SearchEntry, name: &str) -> Option<String> {
    if let Some(item) = contact.attrs.get(name) {
        item.get(0).cloned()
    } else {
        None
    }
}
