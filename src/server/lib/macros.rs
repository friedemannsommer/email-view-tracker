#[macro_export]
macro_rules! static_asset_route {
    ($path: literal, $name: literal, $mediaType: literal) => {{
        async fn handle_static_asset() -> ::actix_web::HttpResponse {
            let mut response = actix_web::HttpResponse::build(actix_web::http::StatusCode::OK)
                .body(actix_web::web::Bytes::from_static(include_bytes!($name)));

            response.headers_mut().insert(
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::HeaderValue::from_static($mediaType),
            );

            response
        }

        ::actix_web::web::resource($path).route(::actix_web::web::get().to(handle_static_asset))
    }};
}
