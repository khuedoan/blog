static ATOM_XML: &str = include_str!(concat!(env!("OUT_DIR"), "/atom.xml"));

pub async fn atom_feed() -> axum::response::Response {
    use axum::response::IntoResponse;

    (
        [(
            axum::http::header::CONTENT_TYPE,
            "application/atom+xml; charset=utf-8",
        )],
        ATOM_XML,
    )
        .into_response()
}
