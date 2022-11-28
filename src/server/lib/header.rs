pub fn get_default_headers_middleware() -> actix_web::middleware::DefaultHeaders {
    actix_web::middleware::DefaultHeaders::new()
        .add((
            actix_web::http::header::CONTENT_SECURITY_POLICY,
            "default-src 'none'; block-all-mixed-content; img-src data: 'self'; style-src 'self'; prefetch-src 'self'; font-src 'self'; frame-ancestors 'none'",
        ))
        .add((actix_web::http::header::REFERRER_POLICY, "no-referrer"))
        .add((actix_web::http::header::X_FRAME_OPTIONS, "SAMEORIGIN"))
        .add((actix_web::http::header::X_CONTENT_TYPE_OPTIONS, "nosniff"))
}
