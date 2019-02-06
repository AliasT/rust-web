use rocket_contrib::json::JsonValue;
use serde::Serialize;
pub struct SuccessResponse {}
pub struct ErrorResponse {}

impl SuccessResponse {
    pub fn new<T: Serialize>(data: T) -> JsonValue {
        json!({
          "code": 200,
          "message": "success",
          "data": data
        })
    }
}

impl ErrorResponse {
    pub fn new<T: Serialize>(data: T) -> JsonValue {
        json!({
          "code": 5000,
          "message": data,
        })
    }

    pub fn unknonw() -> JsonValue {
        json!({
            "code": 5100,
            "message": "未知错误",
        })
    }

    pub fn uncomplete(v: String) -> JsonValue {
        json!({
            "code": 5001,
            "message": format!("{} 不能为空", v)
        })
    }
}
