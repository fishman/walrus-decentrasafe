use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::sqlite::SqliteConnection;
use diesel::{sql_query, RunQueryDsl};
use dotenvy::dotenv;
use env_logger::Env;
use std::env;
use std::time::Duration;
use uuid::Uuid;

mod models;
mod schema;
use models::{Blob, Manifest, NewManifest};
use schema::blobs::dsl::blobs;
use schema::manifests::dsl::manifests;

#[derive(Debug)]
pub struct ConnectionOptions {
    enable_wal: bool,
    enable_foreign_keys: bool,
    busy_timeout: Option<Duration>,
}

//use schema::manifests::dsl::manif;
//use schema::blobs::dsl::*;

#[derive(Clone)]
struct RegistryData {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl diesel::r2d2::CustomizeConnection<SqliteConnection, diesel::r2d2::Error>
    for ConnectionOptions
{
    fn on_acquire(&self, conn: &mut SqliteConnection) -> Result<(), diesel::r2d2::Error> {
        if self.enable_wal {
            sql_query("PRAGMA journal_mode = WAL")
                .execute(conn)
                .map_err(diesel::r2d2::Error::QueryError)?;
            sql_query("PRAGMA synchronous = NORMAL")
                .execute(conn)
                .map_err(diesel::r2d2::Error::QueryError)?;
        }
        if self.enable_foreign_keys {
            sql_query("PRAGMA foreign_keys = ON")
                .execute(conn)
                .map_err(diesel::r2d2::Error::QueryError)?;
        }
        if let Some(d) = self.busy_timeout {
            sql_query(&format!("PRAGMA busy_timeout = {};", d.as_millis()))
                .execute(conn)
                .map_err(diesel::r2d2::Error::QueryError)?;
        }
        Ok(())
    }
}

fn establish_connection_pool() -> Pool<ConnectionManager<SqliteConnection>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);

    let connection_options = ConnectionOptions {
        enable_wal: true,
        enable_foreign_keys: true,
        busy_timeout: Some(Duration::from_secs(5)),
    };

    let pool = Pool::builder()
        .max_size(1)
        .connection_customizer(Box::new(connection_options))
        .build(manager)
        .expect("Failed to create pool.");

    pool
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

    match data.pool.get() {
        Ok(mut conn) => {
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

async fn fetch_blob(data: web::Data<RegistryData>, digest: web::Path<String>) -> impl Responder {
    let mut conn = data.pool.get().expect("Failed to get DB connection");

    let blob = blobs
        .filter(schema::blobs::uuid.eq(digest.as_str()))
        .first::<Blob>(&mut conn);

    match blob {
        Ok(blob) => HttpResponse::Ok().body(blob.data),
        Err(_) => HttpResponse::NotFound().json(serde_json::json!({ "error": "Blob not found" })),
    }
}

async fn upload_manifest(
    data: web::Data<RegistryData>,
    name: web::Path<String>,
    reference: web::Path<String>,
    body: web::Bytes,
) -> impl Responder {
    let manifest = NewManifest {
        name: &name,
        reference: &reference,
        content: &body.to_vec(),
    };

    let conn_result: Result<
        PooledConnection<ConnectionManager<SqliteConnection>>,
        diesel::r2d2::PoolError,
    > = data.pool.get();

    match conn_result {
        Ok(mut conn) => {
            diesel::insert_into(manifests)
                .values(&manifest)
                .execute(&mut conn)
                .expect("Error inserting manifest");

            HttpResponse::Created().json(
                serde_json::json!({ "name": name.to_string(), "reference": reference.to_string() }),
            )
        }
        Err(e) => {
            eprintln!("Error getting connection from pool: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn fetch_manifest(
    data: web::Data<RegistryData>,
    name: web::Path<String>,
    reference: web::Path<String>,
) -> impl Responder {
    let mut conn = data.pool.get().expect("Failed to get DB connection");

    let manifest = manifests
        .filter(schema::manifests::name.eq(name.as_str()))
        .filter(schema::manifests::reference.eq(reference.as_str()))
        .first::<Manifest>(&mut conn);

    match manifest {
        Ok(manifest) => HttpResponse::Ok().body(manifest.content),
        Err(_) => {
            HttpResponse::NotFound().json(serde_json::json!({ "error": "Manifest not found" }))
        }
    }
}

async fn get_tags(data: web::Data<RegistryData>, name: web::Path<String>) -> impl Responder {
    let mut conn = data.pool.get().expect("Failed to get DB connection");

    let tag_list: Vec<String> = manifests
        .filter(schema::manifests::name.eq(name.as_str()))
        .select(schema::manifests::reference)
        .load::<String>(&mut conn)
        .expect("Error loading tags");

    HttpResponse::Ok().json(serde_json::json!({
        "name": name.to_string(),
        "tags": tag_list
    }))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let pool = establish_connection_pool();

    let registry_data = web::Data::new(RegistryData { pool });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new(
                "%a %r %s %b \"%{Referer}i\" \"%{User-Agent}i\" %Dms",
            ))
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
            .route("/v2/{name}/blobs/{digest}", web::get().to(fetch_blob))
            .route(
                "/v2/{name}/manifests/{reference}",
                web::get().to(fetch_manifest),
            )
            .route(
                "/v2/{name}/manifests/{reference}",
                web::put().to(upload_manifest),
            )
            .route("/v2/{name}/tags/list", web::get().to(get_tags))
    })
    .bind("127.0.0.1:8090")?
    .run()
    .await
}
