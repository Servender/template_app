use actix_web::{
    web,
    HttpResponse,
    Result,
    error,
    Error
};
use tera::{Context};

use super::{TEMPLATE, get_template_path};

async fn index_page() -> Result<HttpResponse, Error> {
    let ctx = Context::new();

    let rendered = &TEMPLATE.render(&get_template_path("app", "main"), &ctx)
            .map_err(|e| {
                error!("Error Template main page: {}", e);
                error::ErrorInternalServerError(e)
            })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(index_page));
    cfg.route("/", web::get().to(index_page));
}