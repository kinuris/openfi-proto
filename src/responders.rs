pub mod file {
    use std::{collections::hash_map::DefaultHasher, hash::Hasher, path::PathBuf};

    use axum::{
        body::{Body, StreamBody},
        http::{header, HeaderMap, HeaderValue, Request, StatusCode},
        response::IntoResponse,
    };
    use chrono::{DateTime, Local};
    use std::hash::Hash;
    use tokio_util::io::ReaderStream;

    use crate::extensions::PathBufDetemineMimeExt;

    pub async fn open(
        path: PathBuf,
        req: &Request<Body>,
    ) -> Result<(HeaderMap, impl IntoResponse), StatusCode> {
        let file = tokio::fs::File::open(&path)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)?;

        let stream = ReaderStream::new(file);
        let body = StreamBody::new(stream);

        let mut headers = HeaderMap::new();

        if let Ok(metadata) = tokio::fs::metadata(&path).await {
            if let Ok(time) = metadata.modified() {
                let mut hasher = DefaultHasher::new();
                time.hash(&mut hasher);

                let hash = hasher.finish();
                let match_header = req.headers().get("If-None-Match");

                if match_header.is_some()
                    && match_header.unwrap().as_bytes() == hash.to_string().as_bytes()
                {
                    return Err(StatusCode::NOT_MODIFIED);
                }

                headers.append(header::ETAG, HeaderValue::from(hash));

                let dt: DateTime<Local> = time.into();
                headers.append(
                    header::LAST_MODIFIED,
                    HeaderValue::from_str(&dt.to_rfc2822()).unwrap(),
                );
            };
        };

        let mime_type = path.get_mime_type();

        headers.append(
            header::CONTENT_TYPE,
            HeaderValue::from_str(mime_type.essence_str()).unwrap(),
        );

        Ok((headers, body))
    }

    pub async fn simple_open(path: PathBuf) -> Result<(HeaderMap, impl IntoResponse), StatusCode> {
        let file = tokio::fs::File::open(&path)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)?;

        let stream = ReaderStream::new(file);
        let body = StreamBody::new(stream);

        let mut headers = HeaderMap::new();
        let mime_type = path.get_mime_type();

        headers.append(
            header::CONTENT_TYPE,
            HeaderValue::from_str(mime_type.essence_str()).unwrap(),
        );

        Ok((headers, body))
    }
}
