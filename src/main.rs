use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, Pool, PooledConnection};
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::env;
use uuid::Uuid;

mod models;
mod schema;
use models::Blob;
use schema::blobs::dsl::blobs;
//use schema::blobs::dsl::*;

#[derive(Clone)]
struct RegistryData {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

fn establish_connection_pool() -> Pool<ConnectionManager<SqliteConnection>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

async fn check_registry() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "message": "OCI Registry v2" }))
}

async fn start_blob_upload(data: web::Data<RegistryData>) -> impl Responder {
    let upload_uuid = Uuid::new_v4().to_string();

    let new_blob = Blob {
        uuid: upload_uuid.clone(),
        data: vec![],
    };

    let conn_result: Result<
        PooledConnection<ConnectionManager<SqliteConnection>>,
        diesel::r2d2::PoolError,
    > = data.pool.get();

    match conn_result {
        Ok(mut conn) => {
            // Insert the new blob into the database
            diesel::insert_into(blobs)
                .values(&new_blob)
                .execute(&mut conn)
                .expect("Error inserting new blob");

            HttpResponse::Accepted()
                .append_header(("Location", format!("/v2/blobs/uploads/{}", upload_uuid)))
                .append_header(("Docker-Upload-UUID", upload_uuid.clone()))
                .json(serde_json::json!({ "uuid": upload_uuid }))
        }
        Err(e) => {
            eprintln!("Error getting connection from pool: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn complete_blob_upload(
    data: web::Data<RegistryData>,
    uuid: web::Path<String>,
    body: web::Bytes,
) -> impl Responder {
    let target = blobs.filter(schema::blobs::uuid.eq(uuid.as_str()));
    let conn_result: Result<
        PooledConnection<ConnectionManager<SqliteConnection>>,
        diesel::r2d2::PoolError,
    > = data.pool.get();

    match conn_result {
        Ok(mut conn) => {
            // Find the blob and update it with the uploaded data
            let updated = diesel::update(target)
                .set(schema::blobs::data.eq(body.to_vec()))
                .execute(&mut conn)
                .expect("Failed to update blob data");

            if updated == 1 {
                HttpResponse::Created().json(serde_json::json!({ "uuid": uuid.to_string() }))
            } else {
                HttpResponse::NotFound().json(serde_json::json!({ "error": "Blob not found" }))
            }
        }
        Err(e) => {
            eprintln!("Error getting connection from pool: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn fetch_manifest(name: web::Path<String>) -> impl Responder {
    // Dummy manifest response
    let manifest = serde_json::json!({
        "schemaVersion": 2,
        "name": name.clone(),
        "layers": []
    });
    HttpResponse::Ok().json(manifest)
}

fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let pool = establish_connection_pool();

    let registry_data = web::Data::new(RegistryData { pool });

    HttpServer::new(move || {
        App::new()
            .app_data(registry_data.clone())
            .route("/v2/", web::get().to(check_registry))
            .route(
                "/v2/{name}/blobs/uploads/",
                web::post().to(start_blob_upload),
            )
            .route(
                "/v2/{name}/blobs/uploads/{uuid}",
                web::put().to(complete_blob_upload),
            )
            .route(
                "/v2/{name}/manifests/{reference}",
                web::get().to(fetch_manifest),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
