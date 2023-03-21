# Changelog

## [v1.2.0](https://github.com/fastly/fastly-rust/releases/tag/release/v1.2.0) (2023-03-21)

## Bug fixes

- fix(purge): switch authentication type to 'token'

## Enhancements

- feat(events): implement 'filter_created_at' property
- feat(mutual-authentication): implement 'include' property
- feat(object-store): implement new Object Store API endpoints
- feat(settings): implement Service Settings 'update' endpoint

## Documentation

- docs(backend): keepalive_time
- docs(pop): region, shield, latitude, longitude
- docs(product-enablement): brotli_compression
- docs(resource): terminology
- docs(results): fanout properties
- docs(tls/subscriptions): new 'failed' state
- docs(user): 'login' modification note removed

## [v1.0.0](https://github.com/fastly/fastly-rust/releases/tag/release/v1.0.0) (2022-12-15)

**Enhancements:**

* New interface from code-generated API client [#43](https://github.com/fastly/fastly-rust/pull/3) 
  * [Blog post: Better Fastly API clients with OpenAPI Generator](https://dev.to/fastly/better-fastly-api-clients-with-openapi-generator-3lno)
  * [Documentation](https://github.com/fastly/fastly-rust#documentation-for-api-endpoints)
  * [Unsupported API endpoints](https://github.com/fastly/fastly-rust#issues)
