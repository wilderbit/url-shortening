table! {
    url_shortening_url (id) {
        id -> Int4,
        hash -> Text,
        alias -> Nullable<Text>,
        original_url -> Text,
        expired_on -> Timestamptz,
        created_on -> Timestamptz,
    }
}
