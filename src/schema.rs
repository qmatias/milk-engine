table! {
    comments (id) {
        id -> Integer,
        post_time -> Timestamp,
        ip_address -> Nullable<Text>,
        author -> Text,
        content -> Text,
        from_engineer -> Bool,
    }
}
