This Service will contain code related to url shortening
- 

## Services
- URL shortening service, will be a main server for providing services for creating and getting 
- Key generation service, This service will provide Keys to newly generated short urls.
- KAFKA service, for analytics purpose that how many times url has been accessed.
- Caching service, This service will cache frequently accessed urls. Probably LRU.

## APIs 
#### Create API
- Arguments(String: actual_url, String: api_key, Optional<Time>: expiry_time, Optional<String>: alias)
- Return type(String: Short URL)

This api will create short url for provided actual_url

#### Get API
- Arguments(String: short_url)
- Return type(String: Actual URL) 

this api will redirect to the actual url, if present otherwise will return an error page with status 404.  

This api will get the short url that is provided by the service and redirect to the actual url.

We need to make this service highly available, every request will be treated as event and will serve as fast as possible.
