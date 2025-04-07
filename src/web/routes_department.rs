use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use sea_orm::{ColumnTrait, EntityTrait, QuerySelect};
use serde::Serialize;

use entity::{contact, department};

use crate::error::Result;
use crate::AppState;

pub fn routes_department(state: AppState) -> Router {
    Router::new()
        .route("/departments", get(get_departments))
        .with_state(state)
}

#[derive(Serialize, PartialEq, Eq, PartialOrd, Ord)]
struct Department {
    id: i32,
    name: String,
    count: i64,
}

async fn get_departments(State(state): State<AppState>) -> Result<Json<Vec<Department>>> {
    let department_count: Vec<(i64, i64)> = contact::Entity::find()
        .select_only()
        .column(contact::Column::DepartmentId)
        .column_as(contact::Column::DepartmentId.count(), "count")
        .group_by(contact::Column::DepartmentId)
        .having(contact::Column::DepartmentId.is_not_null())
        .into_tuple()
        .all(&state.conn)
        .await?;

    let total = department_count
        .iter()
        .map(|(_, v)| *v)
        .collect::<Vec<i64>>()
        .iter()
        .sum::<i64>();

    let mut departments = department::Entity::find()
        .all(&state.conn)
        .await?
        .iter()
        .map(|m| Department {
            id: m.id,
            name: m.name.to_owned(),
            count: department_count
                .iter()
                .find(|&&d| d.0 == m.id as i64)
                .unwrap_or(&(m.id as i64, 0))
                .1,
        })
        .filter(|f| f.count != 0)
        .collect::<Vec<Department>>();

    departments.sort_by(|a, b| a.name.cmp(&b.name));
    departments.insert(
        0,
        Department {
            id: 0,
            name: "Все отделы".to_owned(),
            count: total,
        },
    );

    Ok(Json(departments))
}
