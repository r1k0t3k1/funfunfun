use actix_web::{HttpResponse, HttpResponseBuilder, Responder, body::BoxBody, http::{StatusCode, header::ContentType}};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum ResponseResult {
    OK,
    ERROR,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResponseBody<T: Serialize> {
    result: ResponseResult,
    status_code: u16,
    data: T,
}

impl<T: Serialize> ResponseBody<T> {
    pub fn new(result: ResponseResult, status_code: StatusCode, data: T) -> Self {
        Self { result, status_code: status_code.as_u16(), data }
    }

    pub fn ok(status_code: StatusCode, data: T) -> Self {
        Self { result: ResponseResult::OK, status_code: status_code.as_u16(), data }
    }

    pub fn error(status_code: StatusCode, data: T) -> Self {
        Self { result: ResponseResult::ERROR, status_code: status_code.as_u16(), data }
    }
}

impl<T: Serialize> Responder for ResponseBody<T> {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let Ok(body_str) = serde_json::to_string(&self) else {
            // コンストラクタでStatusCodeからu16生成しているためエラーは無視できる
            return HttpResponseBuilder::new(StatusCode::from_u16(self.status_code).unwrap())
                .content_type(ContentType::json())
                .body(r#"{"result":"ERROR","status_code":500,"data":"Failed to parse struct(response body)"}"#);
        };

        HttpResponseBuilder::new(StatusCode::from_u16(self.status_code).unwrap())
            .content_type(ContentType::json())
            .body(body_str)
    }
}
