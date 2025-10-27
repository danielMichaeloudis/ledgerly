mod components;
mod pages;

use std::fs;

use axum::{
    body::Body,
    extract::Request,
    http::{HeaderMap, Response, StatusCode},
    middleware::{self, Next},
    routing::{get, post},
    Json, Router,
};
use css_helper::Css;
use pages::{login_page, page};

use crate::{
    api_bridge::ApiBridge,
    models::Item,
    website::{
        components::{add_transaction, edit_transaction, import_transactions, item_row},
        pages::{analytics_page, authorised_page, signup_page, table_page},
    },
    AppState,
};

pub(crate) fn website_routes() -> Router<AppState> {
    let login_routes = Router::new()
        .route("/login", get(async || page(login_page())))
        .route("/signup", get(async || page(signup_page())))
        .route_layer(middleware::from_fn(check_logged_in));

    Router::new()
        .route(
            "/",
            get(async |req: Request| authorised_page(table_page, req).await),
        )
        .route(
            "/analytics",
            get(async |req: Request| authorised_page(analytics_page, req).await),
        )
        .route(
            "/table",
            get(async |req: Request| authorised_page(table_page, req).await),
        )
        .route(
            "/components/add_single_transaction",
            get(async || add_transaction()),
        )
        .route("/components/edit_transaction", post(edit_transaction))
        .route("/components/import_transactions", post(import_transactions))
        .route(
            "/components/item-row",
            post(async |item: Json<Option<Item>>| item_row(item)),
        )
        .route_layer(middleware::from_fn(auth))
        .merge(login_routes)
}

async fn auth(req: Request, next: Next) -> Result<Response<Body>, (StatusCode, String)> {
    let api_bridge = ApiBridge::new()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let uri = req.uri().to_string();
    let token = match get_cookie(&req, "token") {
        Some(token) => token,
        None => return Ok(redirect_to_login(&uri)),
    };

    match api_bridge.test_token(&token).await.is_ok() {
        true => Ok(next.run(req).await),
        false => Ok(redirect_to_login(&uri)),
    }
}

async fn check_logged_in(req: Request, next: Next) -> Result<Response<Body>, (StatusCode, String)> {
    let api_bridge = ApiBridge::new()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let uri = req.uri().to_string();
    let token = match get_cookie(&req, "token") {
        Some(token) => token,
        None => return Ok(next.run(req).await),
    };

    match api_bridge.test_token(&token).await.is_ok() {
        true => Ok(redirect_to_home(&uri)),
        false => Ok(next.run(req).await),
    }
}

fn get_cookie(req: &Request, cookie: &str) -> Option<String> {
    get_cookie_from_headers(req.headers(), cookie)
}

fn get_cookie_from_headers(headers: &HeaderMap, cookie: &str) -> Option<String> {
    let cookies: Vec<&str> = match headers.get("cookie") {
        Some(cookie) => match cookie.to_str() {
            Ok(cookie) => cookie,
            Err(_) => return None,
        }
        .split(";")
        .collect(),
        None => return None,
    };

    for cookie_pair in cookies {
        let (key, val) = cookie_pair.split_once("=").unwrap_or(("", ""));
        if key == cookie {
            return Some(val.to_string());
        }
    }
    None
}

fn redirect_to_login(_from: &str) -> Response<Body> {
    let mut res = Response::new(Body::empty());
    *res.status_mut() = StatusCode::FOUND;
    res.headers_mut()
        .append("Location", "/login".parse().unwrap());
    res
}

fn redirect_to_home(_from: &str) -> Response<Body> {
    let mut res = Response::new(Body::empty());
    *res.status_mut() = StatusCode::FOUND;
    res.headers_mut().append("Location", "/".parse().unwrap());
    res
}
