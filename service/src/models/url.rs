use super::prelude::*;
use crate::diesel::ExpressionMethods;
use diesel::RunQueryDsl;

pub fn create(
    conn: &diesel::pg::PgConnection,
    hash: &str,
    alias: &Option<String>,
    url: &str,
    expired_on: &DateTime<Utc>,
) -> Result<(), diesel::result::Error> {
    use super::schema::url_shortening_url;
    diesel::insert_into(url_shortening_url::table)
        .values((
            url_shortening_url::hash.eq(hash),
            url_shortening_url::alias.eq(alias),
            url_shortening_url::original_url.eq(url),
            url_shortening_url::expired_on.eq(expired_on),
            url_shortening_url::created_on.eq(Utc::now()),
        ))
        .execute(conn)
        .map(|_: usize| ())
}
