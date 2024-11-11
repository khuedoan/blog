use axum::{
    extract::Path,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use include_dir::{include_dir, Dir};

static PUBLIC_FILES: Dir = include_dir!("public");

pub async fn file(Path(path): Path<String>) -> Response {
    PUBLIC_FILES.get_file(&path).map_or_else(
        // TODO generic error handler
        || (StatusCode::NOT_FOUND).into_response(),
        |file| {
            let content_type = match path.split('.').last() {
                // https://developer.mozilla.org/en-US/docs/Web/HTTP/MIME_types
                Some("css") => "text/css",
                Some("js") => "application/javascript",
                Some("jpg") | Some("jpeg") => "image/jpeg",
                Some("png") => "image/png",
                _ => "application/octet-stream",
            };
            (
                [
                    (header::CONTENT_TYPE, content_type),
                    (header::CACHE_CONTROL, "public, max-age=31536000"),
                ],
                file.contents(),
            )
                .into_response()
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

        let content_type = response.headers().get(header::CONTENT_TYPE);
        assert_eq!(
            content_type,
            Some(&header::HeaderValue::from_static("application/javascript"))
        );

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

        let content_type = response.headers().get(header::CONTENT_TYPE);
        assert_eq!(
            content_type,
            Some(&header::HeaderValue::from_static("text/css"))
        );

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
    async fn file_gpg() {
        let response = file(Path("gpg".to_string())).await;
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
        .contains("BEGIN PGP PUBLIC KEY BLOCK"));
    }

    #[tokio::test]
    async fn file_nested_image() {
        let response = file(Path("images/avatar.jpg".to_string())).await;
        assert_eq!(response.status(), StatusCode::OK);

        let content_type = response.headers().get(header::CONTENT_TYPE);
        assert_eq!(
            content_type,
            Some(&header::HeaderValue::from_static("image/jpeg"))
        );
    }

    #[tokio::test]
    async fn file_not_found() {
        let notfound = file(Path("dummy.txt".to_string())).await;
        assert_eq!(notfound.status(), StatusCode::NOT_FOUND);
    }
}
