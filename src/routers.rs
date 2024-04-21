use crate::handlers;
use axum::{
    routing::{get, post},
    Router,
};

pub fn app() -> Router {
    Router::new()
        .route("/", get(handlers::root))
        .route("/user", post(handlers::create_user))
}

#[cfg(test)]
mod tests {
    use axum::{body::Body, extract::Request, http};
    use tower::ServiceExt;

    use crate::routers;

    #[tokio::test]
    async fn get_hello() {
        let req = Request::builder().uri("/").body(Body::empty()).unwrap();

        let res = routers::app().oneshot(req).await.unwrap();

        let body = axum::body::to_bytes(res.into_body(), usize::MAX)
            .await
            .unwrap()
            .to_vec();

        let text = String::from_utf8(body).unwrap();

        assert_eq!(text, "Hello world".to_string());
    }

    #[tokio::test]
    async fn get_user() {
        let app = routers::app();

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/user")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        r#"
                        {
                            "user_name": "ryo",
                            "email": "ryo@gmail.com"
                        }
                        "#
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        let body = response.into_body();
        let body = axum::body::to_bytes(body, usize::MAX).await.unwrap();
        
        let text = String::from_utf8(body.to_vec()).unwrap();
        let ans =  r#"{"id":1,"user_name":"ryo","email":"ryo@gmail.com"}"#.to_string();

        assert_eq!(text,ans);
    }

}
