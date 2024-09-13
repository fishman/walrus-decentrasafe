// @generated automatically by Diesel CLI.

diesel::table! {
    blobs (uuid) {
        uuid -> Nullable<Text>,
        data -> Binary,
    }
}
