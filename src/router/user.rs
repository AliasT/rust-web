
use rocket::request::Form;
use rocket_contrib::json::{JsonValue};
use crate::db::user::{User, NewUser};
use crate::res::{SuccessResponse, ErrorResponse};
use rocket::request::{FormError, FormDataError, FormParseError};

#[post("/signup", data="<user>")]
pub fn signup(conn: crate::DbConn, user: Result<Form<NewUser>, FormError>) -> JsonValue {
  match user {
    Ok(user_payload) => {
        let new_user = NewUser::create(&conn, user_payload.into_inner());
		SuccessResponse::new(new_user)
    },
	// 字段缺失
	Err(FormDataError::Parse(e, _)) => {
		match e {
			FormParseError::Missing(v) => {
                ErrorResponse::new(format!("{} 不能为空", v.as_str()))
			},
			_ => {
				ErrorResponse::new("未知错误")
			}
		}
	},
	// 其他错误
	_ => {
		ErrorResponse::new("未知错误")
	}
  }
}

#[get("/users")]
pub fn get_all_users(conn: crate::DbConn) -> JsonValue {
    SuccessResponse::new(User::get(&conn))
}
