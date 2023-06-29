use std::env;

use axum::extract::{Path, Query, State};
use axum::routing::get;
use axum::{Json, Router};
use sea_orm::{
    ColumnTrait, EntityTrait, FromQueryResult, JoinType, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait, RelationTrait, Select,
};
use serde::{Deserialize, Serialize};

use entity::{
    city, company, contact, country, department, job_title, postal_code, state, street_address,
};

use crate::error::{Error, Result};
use crate::AppState;

pub fn routes_contact(state: AppState) -> Router {
    Router::new()
        .route("/contacts", get(get_contacts))
        .route("/contact/:id", get(get_contact))
        .with_state(state)
}

#[derive(Serialize)]
struct ContactResponse {
    total: u64,
    page_size: u64,
    page: u64,
    data: Vec<Contact>,
}

#[derive(Serialize, FromQueryResult)]
pub struct Contact {
    id: i32,
    name: String,
    photo: bool,
    employee_number: Option<String>,
    mail: Option<String>,
    telephone_number: Option<String>,
    mobile: Option<String>,
    birthdate: Option<String>,
    company: Option<String>,
    department: Option<String>,
    job_title: Option<String>,
    country: Option<String>,
    postal_code: Option<String>,
    state: Option<String>,
    city: Option<String>,
    street_address: Option<String>,
}

#[derive(Deserialize)]
struct Params {
    page: u64,
    name: Option<String>,
    dep_id: Option<u32>,
}

async fn get_contacts(
    State(state): State<AppState>,
    Query(params): Query<Params>,
) -> Result<Json<ContactResponse>> {
    let page_size = env::var("PAGE_SIZE")
        .expect("PAGE_SIZE is not set in .env file")
        .parse()
        .unwrap();

    let Params { name, dep_id, page } = params;

    let pages = create_query(contact::Entity::find())
        .apply_if(name, |query, v| {
            query.filter(contact::Column::Name.like(&format!("%{}%", v)))
        })
        .apply_if(dep_id, |query, v| {
            query.filter(contact::Column::DepartmentId.eq(v))
        })
        .into_model::<Contact>()
        .paginate(&state.conn, page_size);

    let total = pages.num_pages().await?;
    let contacts = pages.fetch_page(page).await?;

    Ok(Json(ContactResponse {
        total,
        page_size,
        page,
        data: contacts,
    }))
}

async fn get_contact(State(state): State<AppState>, Path(id): Path<i32>) -> Result<Json<Contact>> {
    let contact = create_query(contact::Entity::find_by_id(id))
        .into_model::<Contact>()
        .one(&state.conn)
        .await?;

    if let Some(contact) = contact {
        Ok(Json(contact))
    } else {
        Err(Error::ContactNotFound)
    }
}

fn create_query(contact: Select<contact::Entity>) -> Select<contact::Entity> {
    contact
        .select_only()
        .columns([
            contact::Column::Id,
            contact::Column::Name,
            contact::Column::Photo,
            contact::Column::EmployeeNumber,
            contact::Column::Mail,
            contact::Column::TelephoneNumber,
            contact::Column::Mobile,
            contact::Column::Birthdate,
        ])
        .column_as(department::Column::Name, "department")
        .column_as(job_title::Column::Name, "job_title")
        .column_as(company::Column::Name, "company")
        .column_as(country::Column::Name, "country")
        .column_as(postal_code::Column::Name, "postal_code")
        .column_as(state::Column::Name, "state")
        .column_as(city::Column::Name, "city")
        .column_as(street_address::Column::Name, "street_address")
        .join(JoinType::LeftJoin, contact::Relation::Department.def())
        .join(JoinType::LeftJoin, contact::Relation::JobTitle.def())
        .join(JoinType::LeftJoin, contact::Relation::Company.def())
        .join(JoinType::LeftJoin, contact::Relation::Country.def())
        .join(JoinType::LeftJoin, contact::Relation::PostalCode.def())
        .join(JoinType::LeftJoin, contact::Relation::State.def())
        .join(JoinType::LeftJoin, contact::Relation::City.def())
        .join(JoinType::LeftJoin, contact::Relation::StreetAddress.def())
        .order_by_asc(contact::Column::Name)
}
