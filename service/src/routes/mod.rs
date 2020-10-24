pub mod create;

pub async fn create_url(api_key: String, url: String, expiry_time: Option<String>) -> String {
    // TODO: api_key check will implement later
    // this function will get the actual url and return the short url
    //
    return "".to_string();
}

pub async fn get_url(url: String) -> String {
    // This function will redirect to the actual url
    // Other It will return 404 status if it not found
    // TODO: caching, LRU later, analytics
    "".to_string()
}
