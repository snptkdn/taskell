
diesel::table! {
    users (id) {
        id -> Unsigned<Bigint>,
        name -> Varchar,
        encrypted_pass -> Varchar,
    }
}
