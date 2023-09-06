// get user's Ip Address

use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path, Query, ServiceConfig},
    HttpResponse, Responder,
};

pub(super) fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config
            .service(collect);
    }
}

/// Get user's Ip Address
///
/// Get user's ip address
///
/// One could call the api endpoint with following curl.
/// ```text
/// curl localhost:8080/collect
/// ```
#[utoipa::path(
    responses(
        (status = 200, description = "Get user's Ip Address", body = [collect])
    )
)]
#[get("/collect")]
pub async fn collect(req: actix_web::HttpRequest) -> impl Responder {
    if let Some(val) = req.peer_addr() {
        println!("User IP Address {:?}", val.ip());
    };
    HttpResponse::Ok().json(req.peer_addr())
}