use crate::schema::blobs;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Insertable, Serialize)]
#[diesel(table_name = blobs)]
pub struct Blob {
    pub uuid: String,
    pub data: Vec<u8>,
}
