// @generated automatically by Diesel CLI.

diesel::table! {
    done_tasks (id) {
        id -> Unsigned<Bigint>,
        point -> Nullable<Integer>,
        done_date -> Datetime,
    }
}

diesel::table! {
    login_info (id) {
        id -> Unsigned<Bigint>,
        mac_address -> Varchar,
        user_id -> Nullable<Integer>,
    }
}

diesel::table! {
    tasks (id) {
        id -> Unsigned<Bigint>,
        point -> Nullable<Integer>,
        user_id -> Nullable<Integer>,
        title -> Varchar,
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
    done_tasks,
    login_info,
    tasks,
    users,
);
