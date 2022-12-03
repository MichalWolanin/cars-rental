#[macro_use] extern crate rocket;
#[cfg(test)] mod tests;

use rocket::serde::json::{Json, Value, json};
use rocket::serde::{Serialize, Deserialize};
use rocket::http::Status;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Car {
  id: Option<u16>,
  brand: String,
}

#[get("/cars")]
fn getCars() -> Option<Json<Vec<Car>>> {
  Some(Json(vec![Car {
    id: Some(1),
    brand: String::from("Volvo"),
  }, Car {
    id: Some(2),
    brand: String::from("Mercedes"),
  }]))
  // RawJson("{ 'id': 1, 'brand': 'Volvo' }, { 'id': 2, 'brand': 'Mercedes' }")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![getCars])
}
