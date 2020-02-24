table! {
    districts (id) {
        id -> Integer,
        name -> Varchar,
        code -> Nullable<Varchar>,
        parent_id -> Nullable<Bigint>,
        record_time -> Datetime,
    }
}
