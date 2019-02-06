use crate::db::user::{NewUser, SignupUser, User};
use crate::res::{ErrorResponse, SuccessResponse};
use rocket::request::{FormDataError, FormError, FormParseError, LenientForm};
use rocket_contrib::json::JsonValue;

#[post("/signup", data = "<user>")]
pub fn signup(conn: crate::DbConn, user: Result<LenientForm<SignupUser>, FormError>) -> JsonValue {
    match user {
        Ok(user_payload) => {
            let new_user = NewUser::create(&conn, user_payload.into_inner());
            SuccessResponse::new(new_user)
        }
        // 字段缺失
        Err(FormDataError::Parse(e, _)) => match e {
            FormParseError::Missing(v) => {
                ErrorResponse::uncomplete(String::from(v.as_str()))
            }
            _ => ErrorResponse::unknonw(),
        },
        // 其他错误
        _ => ErrorResponse::unknonw(),
    }
}

#[get("/users")]
pub fn get_all_users(conn: crate::DbConn) -> JsonValue {
    SuccessResponse::new(User::get(&conn))
}
