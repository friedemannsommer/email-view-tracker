#[macro_export]
macro_rules! static_asset_route {
    ($path: literal, $asset: expr, $mediaType: literal) => {{
        async fn handle_static_asset(req: actix_web::HttpRequest) -> actix_web::HttpResponse {
            const ASSET: $crate::model::asset::Asset = $asset;

            if let Some(etag_value) = req.headers().get(actix_web::http::header::IF_NONE_MATCH) {
                if let Ok(etag) = etag_value.to_str() {
                    if let Ok(timestamp) = etag.parse::<i64>() {
                        if timestamp == ASSET.last_modified.unix_timestamp() {
                            return actix_web::HttpResponse::NotModified().finish();
                        }
                    }
                }
            }

            actix_web::HttpResponse::build(actix_web::http::StatusCode::OK)
                .insert_header((
                    actix_web::http::header::CONTENT_TYPE,
                    actix_web::http::header::HeaderValue::from_static($mediaType),
                ))
                .insert_header((
                    actix_web::http::header::CACHE_CONTROL,
                    actix_web::http::header::HeaderValue::from_static(
                        "public, stale-while-revalidate, max-age=604800",
                    ),
                ))
                .insert_header((
                    actix_web::http::header::ETAG,
                    ASSET.last_modified.unix_timestamp(),
                ))
                .body(actix_web::web::Bytes::from_static(&ASSET.contents))
        }

        actix_web::web::resource($path).route(::actix_web::web::get().to(handle_static_asset))
    }};
}
