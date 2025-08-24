use axum_core::{
    body::Body,
    response::{IntoResponse, Response},
};

use crate::Document;

impl IntoResponse for Document {
    fn into_response(self) -> Response {
        Response::new(Body::new(self.to_string()))
    }
}
