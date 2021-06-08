table! {
    grades (id) {
        id -> Int4,
        grade -> Float4,
        student_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        role -> Varchar,
    }
}

joinable!(grades -> users (student_id));

allow_tables_to_appear_in_same_query!(
    grades,
    users,
);
