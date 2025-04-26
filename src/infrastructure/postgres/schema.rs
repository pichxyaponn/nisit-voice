// @generated automatically by Diesel CLI.

diesel::table! {
    nisits (id) {
        id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    report_staff_junction (report_id, staff_id) {
        report_id -> Int4,
        staff_id -> Int4,
        assigned_at -> Timestamp,
    }
}

diesel::table! {
    reports (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 100]
        status -> Varchar,
        nisit_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        resolved_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    staff (id) {
        id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(report_staff_junction -> reports (report_id));
diesel::joinable!(report_staff_junction -> staff (staff_id));
diesel::joinable!(reports -> nisits (nisit_id));

diesel::allow_tables_to_appear_in_same_query!(
    nisits,
    report_staff_junction,
    reports,
    staff,
);
