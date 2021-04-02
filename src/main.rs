#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod models;
mod repositories;
mod schema;

use models::*;
use repositories::*;
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use std::error::Error;

embed_migrations!();

#[database("sqlite_path")]
struct DbConn(diesel::SqliteConnection);

#[get("/templates")]
async fn get_templates(conn: DbConn) -> Result<JsonValue, status::Custom<JsonValue>> {
    conn.run(|c| {
        TemplateRepository::load_all(c)
            .map(|templates| json!(templates))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[get("/templates/<id>")]
async fn view_template(id: i32, conn: DbConn) -> Result<JsonValue, status::Custom<JsonValue>> {
    conn.run(move |c| {
        TemplateRepository::find_one(c, id)
            .map(|templates| json!(templates))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[post("/templates", format = "json", data = "<new_template>")]
async fn create_template(
    conn: DbConn,
    new_template: Json<NewTemplate>,
) -> Result<JsonValue, status::Custom<JsonValue>> {
    conn.run(|c| {
        TemplateRepository::create(c, new_template.into_inner())
            .map(|templates| json!(templates))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[put("/templates/<_id>", format = "json", data = "<template>")]
async fn update_template(
    _id: i32,
    conn: DbConn,
    template: Json<Template>,
) -> Result<JsonValue, status::Custom<JsonValue>> {
    conn.run(move |c| {
        TemplateRepository::save(c, template.into_inner())
            .map(|templates| json!(templates))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[delete("/templates/<id>")]
async fn delete_template(id: i32, conn: DbConn) -> Status {
    conn.run(move |c| match TemplateRepository::delete(c, id) {
        true => Status::NoContent,
        false => Status::NotFound,
    })
    .await
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": 404,
        "title": "Not Found",
        "description": "The resource couldn't be found"
    })
}

#[catch(422)]
fn unprocessable_entity() -> JsonValue {
    json!({
        "status": 422,
        "title": "Unprocessable Entity",
        "description": "The request was well-formed but one or more attributes are missing."
    })
}

async fn run_db_migrations(rocket: rocket::Rocket) -> Result<rocket::Rocket, rocket::Rocket> {
    DbConn::get_one(&rocket)
        .await
        .expect("failed to retrieve database connection")
        .run(|c| match embedded_migrations::run(c) {
            Ok(()) => Ok(rocket),
            Err(e) => {
                println!("Failed to run database migrations: {:?}", e);
                Err(rocket)
            }
        })
        .await
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cors = rocket_cors::CorsOptions::default().to_cors()?;

    let _ = rocket::ignite()
        .mount(
            "/",
            routes![
                get_templates,
                view_template,
                create_template,
                update_template,
                delete_template
            ],
        )
        .mount("/", rocket_cors::catch_all_options_routes())
        .register("/", catchers![not_found, unprocessable_entity])
        .attach(DbConn::fairing())
        .attach(AdHoc::on_attach("Database Migrations", run_db_migrations))
        .launch()
        .await;

    Ok(())
}
