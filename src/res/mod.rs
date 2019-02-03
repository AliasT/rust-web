use rocket_contrib::json::{Json, JsonValue,};
use std::collections::HashMap;
use serde::Serialize;
pub struct SuccessResponse {}

impl SuccessResponse {
  pub fn new<T: Serialize>(data: T) -> JsonValue{
    json!({
      "code": 200,
      "message": "success",
      "data": data
    })
  }
}
