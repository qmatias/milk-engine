table! {
    comments (id) {
        id -> Integer,
        post_time -> Timestamp,
        ip_address -> Nullable<Binary>,
        author -> Text,
        content -> Text,
        from_engineer -> Bool,
    }
}
