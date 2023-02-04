// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Unsigned<Bigint>,
        name -> Varchar,
        encrypted_pass -> Varchar,
    }
}
