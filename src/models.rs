use crate::schema::{blobs, manifests};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = blobs)]
pub struct Blob {
    pub uuid: String,
    pub name: String,
    pub digest: Option<String>,
    pub content_length: Option<i32>,
    pub data: Vec<u8>,
    pub walrus_blob_id: Option<String>,
}

//#[derive(Insertable)]
//#[diesel(table_name = blobs)]
//pub struct NewBlob {
//    pub uuid: String,
//    pub name: String,
//    pub sha256digest: Option<String>,
//    pub data: Vec<u8>,
//}

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = manifests)]
pub struct Manifest {
    pub id: i32,
    pub name: String,
    pub reference: String,
    pub content: Vec<u8>, // Store the JSON manifest as a byte array
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = manifests)]
pub struct NewManifest<'a> {
    pub name: &'a str,
    pub reference: &'a str,
    pub content: &'a [u8],
}
