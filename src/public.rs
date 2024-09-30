use axum::{
    extract::Path,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};

// Embed the public files explicitly to avoid adding an extra dependency
const FILES: &[(&str, &[u8], &str)] = &[
    (
        "htmx.min.js",
        include_bytes!("../public/htmx.min.js"),
        "application/javascript",
    ),
    (
        "pico.min.css",
        include_bytes!("../public/pico.min.css"),
        "text/css",
    ),
];

pub async fn file(Path(file): Path<String>) -> Response {
    FILES.iter().find(|(name, _, _)| name == &file).map_or_else(
        || {
            // TODO generic error handler?
            (StatusCode::NOT_FOUND,).into_response()
        },
        |&(_, content, content_type)| {
            ([(header::CONTENT_TYPE, content_type)], content).into_response()
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use http_body_util::BodyExt;

    #[tokio::test]
    async fn file_htmx() {
        let response = file(Path("htmx.min.js".to_string())).await;
        assert_eq!(response.status(), StatusCode::OK);
        assert!(String::from_utf8(
            response
                .into_body()
                .collect()
                .await
                .unwrap()
                .to_bytes()
                .into(),
        )
        .unwrap()
        .contains("htmx-request"));
    }

    #[tokio::test]
    async fn file_picocss() {
        let response = file(Path("pico.min.css".to_string())).await;
        assert_eq!(response.status(), StatusCode::OK);
        assert!(String::from_utf8(
            response
                .into_body()
                .collect()
                .await
                .unwrap()
                .to_bytes()
                .into(),
        )
        .unwrap()
        .contains("Pico CSS"));
    }

    #[tokio::test]
    async fn file_not_found() {
        let notfound = file(Path("dummy.txt".to_string())).await;
        assert_eq!(notfound.status(), StatusCode::NOT_FOUND);
    }
}
