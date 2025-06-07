use actix_web::cookie::Cookie;
use actix_web::http::header::ContentType;
use actix_web::{HttpRequest, HttpResponse};

pub async fn login_form(request: HttpRequest) -> HttpResponse {
    let error_html: String = match request.cookie("_flash") {
        None => "".into(),
        Some(cookie) => {
            format!("<p><i>{}</i></p>", cookie.value())
        }
    };
    let mut respones = HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
        <html lang="en">
        <head>
        <meta http-equiv="content-type" content="text/html; charset=utf-8">
        </head>
        <body>
        {error_html}
        <form action="/login" method="post">
        <label>Username
        <input type="text" name="username" placeholder="Enter Username">
        </label>

        <label>Password
        <input type="password" name="password" placeholder="Enter Password">
        </label>
        <button type="submit">Login</button>
        </form>
        </body>
        </html>"#
        ));
    respones
        .add_removal_cookie(&Cookie::new("_flash", ""))
        .unwrap();
    respones
}
