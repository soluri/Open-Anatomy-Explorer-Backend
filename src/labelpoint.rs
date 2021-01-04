use crate::util;
use crate::util::json_path;
use rocket::{delete, get, post, put};
use rocket_contrib::{json::Json, uuid::Uuid};
use serde::{Deserialize, Serialize};
use std::{env, error::Error};

#[derive(Serialize, Deserialize, Debug)]
pub struct LabelPoint {
    pub id: i32,
    pub color: String,
    pub name: String,
    pub model: String,
    pub vertices: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>,
}

#[post("/", format = "json", data = "<data>")]
pub fn create(data: Json<Vec<LabelPoint>>) -> Result<Json<String>, Box<dyn Error>> {
    put(util::create_uuid(), data)
}

#[put("/<uuid>", format = "json", data = "<data>")]
pub fn put(uuid: Uuid, data: Json<Vec<LabelPoint>>) -> Result<Json<String>, Box<dyn Error>> {
    let data_dir = env::var("LABEL_DATA_DIR").unwrap();
    let data = serde_json::to_string(&data.into_inner())?;
    std::fs::write(dbg!(json_path(&data_dir, &uuid.to_string())), data)?;
    Ok(Json(uuid.to_string()))
}

#[get("/<uuid>")]
pub fn load(uuid: Uuid) -> Result<Json<Vec<LabelPoint>>, Box<dyn Error>> {
    let data_dir = env::var("LABEL_DATA_DIR").unwrap();
    let data = std::fs::read(json_path(&data_dir, &uuid.to_string()))?;
    let string = std::str::from_utf8(&data)?;
    let result = serde_json::from_str(string)?;
    Ok(Json(result))
}

#[delete("/<uuid>")]
pub fn delete(uuid: Uuid) -> Result<(), Box<dyn Error>> {
    let data_dir = env::var("LABEL_DATA_DIR").unwrap();
    std::fs::remove_file(json_path(&data_dir, &uuid.to_string()))?;
    Ok(())
}
