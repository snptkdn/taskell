// @generated automatically by Diesel CLI.

diesel::table! {
    login_info (id) {
        id -> Unsigned<Bigint>,
        mac_address -> Varchar,
        user_id -> Nullable<Integer>,
    }
}

diesel::table! {
    users (id) {
        id -> Unsigned<Bigint>,
        name -> Varchar,
        encrypted_pass -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    login_info,
    users,
);
