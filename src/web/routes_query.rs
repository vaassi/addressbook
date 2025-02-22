use std::collections::HashMap;

use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use entity::contact;

use crate::error::Result;
use crate::AppState;

pub fn routes_query(state: AppState) -> Router {
    Router::new()
        .route("/query/:q", get(get_query))
        .with_state(state)
}

fn convert_layout(query: &str, layout: &str) -> String {
    let ru = HashMap::from([
        ("q", "й"),
        ("w", "ц"),
        ("e", "у"),
        ("r", "к"),
        ("t", "е"),
        ("y", "н"),
        ("u", "г"),
        ("i", "ш"),
        ("o", "щ"),
        ("p", "з"),
        ("[", "х"),
        ("{", "Х"),
        ("]", "ъ"),
        ("}", "Ъ"),
        ("`", "ё"),
        ("~", "Ё"),
        ("a", "ф"),
        ("s", "ы"),
        ("d", "в"),
        ("f", "а"),
        ("g", "п"),
        ("h", "р"),
        ("j", "о"),
        ("k", "л"),
        ("l", "д"),
        (";", "ж"),
        (",", "Ж"),
        ("'", "э"),
        ("\"", "Э"),
        ("z", "я"),
        ("x", "ч"),
        ("c", "с"),
        ("v", "м"),
        ("b", "и"),
        ("n", "т"),
        ("m", "ь"),
        (",", "б"),
        ("<", "Б"),
        (".", "ю"),
        (">", "Ю"),
    ]);

    let en = HashMap::from([
        ("й", "q"),
        ("ц", "w"),
        ("у", "e"),
        ("к", "r"),
        ("е", "t"),
        ("н", "y"),
        ("г", "u"),
        ("ш", "i"),
        ("щ", "o"),
        ("з", "p"),
        ("ф", "a"),
        ("ы", "s"),
        ("в", "d"),
        ("а", "f"),
        ("п", "g"),
        ("р", "h"),
        ("о", "j"),
        ("л", "k"),
        ("д", "l"),
        ("я", "z"),
        ("ч", "x"),
        ("с", "c"),
        ("м", "v"),
        ("и", "b"),
        ("т", "n"),
        ("ь", "m"),
    ]);

    let mut converted: Vec<String> = vec![];

    if layout == "ru" {
        for c in query.chars() {
            if let Some(&value) = ru.get(c.to_string().as_str()) {
                converted.push(value.to_owned());
            }
        }
    }

    if layout == "en" {
        for c in query.chars() {
            if let Some(&value) = en.get(c.to_string().as_str()) {
                converted.push(value.to_owned());
            }
        }
    }

    return converted.into_iter().collect();
}

async fn get_query(
    State(state): State<AppState>,
    Path(q): Path<String>,
) -> Result<Json<Vec<String>>> {
    let q_ru = convert_layout(&q, "ru");
    let q_en = convert_layout(&q, "en");

    let mut names = contact::Entity::find()
        .filter(contact::Column::Name.like(format!("%{}%", q)))
        .all(&state.conn)
        .await?
        .iter()
        .map(|m| m.name.to_owned())
        .collect::<Vec<String>>();

    if !q_ru.is_empty() {
        let mut names_ru = contact::Entity::find()
            .filter(contact::Column::Name.like(format!("%{}%", q_ru)))
            .all(&state.conn)
            .await?
            .iter()
            .map(|m| m.name.to_owned())
            .collect::<Vec<String>>();
        names.append(&mut names_ru);
    }

    if !q_en.is_empty() {
        let mut names_en = contact::Entity::find()
            .filter(contact::Column::Name.like(format!("%{}%", q_en)))
            .all(&state.conn)
            .await?
            .iter()
            .map(|m| m.name.to_owned())
            .collect::<Vec<String>>();

        names.append(&mut names_en);
    }

    Ok(Json(names))
}
