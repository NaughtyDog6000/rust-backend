use axum::{
    routing::{get, post},
    Router,
};

pub mod delete_account;
pub mod signin;
pub mod signout;
pub mod signup;
pub mod test_token;
pub mod update_account_data;

pub fn router() -> Router {
    Router::new()
        .route("/signin", post(signin::signin).get(signin::get_signin))
        .route(
            "/signup",
            get(|| async { "This does NOT support get requests" }).post(signup::create_user),
        )
        .route(
            "/signout_all",
            get(|| async { "this is not done" }).post(signout::signout_all),
        )
        .route(
            "/signout",
            get(|| async { "This route is incomplete" }).post(signout::signout),
        )
        .route(
            "/account/delete_me",
            get(delete_account::delete_account).post(delete_account::delete_account),
        )
        .route(
            "/account/update/date_of_birth",
            get(|| async {
                "to update send token & a date string formatted: YY-MM-DD or YYYY-M-D etc"
            })
            .post(update_account_data::add_date_of_birth),
        )
        .route(
            "/account/update/password",
            get(|| async {
                "to update password send token & new password in body of a POST request"
            })
            .post(update_account_data::update_password),
        )
        .route(
            "/test_token",
            get(|| async { "This does NOT support get requests" }).post(test_token::test_token),
        )
}
