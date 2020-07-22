use actix_web::{
    web::ServiceConfig,
    dev::ServiceResponse,
    http,
    Result,
    middleware::errhandlers::{
        ErrorHandlers,
        ErrorHandlerResponse
    }
};
use actix_http::body::Body;
use actix_files::NamedFile;

use super::get_page_path;

fn render_error_page<B>(
    res: ServiceResponse<B>,
    error_code: &str
) -> Result<ErrorHandlerResponse<B>> {
    let render_page = get_page_path("error", error_code);

    error!(
        "STATUS: {}, REQUEST: {:?}",
        error_code,
        res.request().head()
    );

    let new_resp = NamedFile::open(render_page)?
        .set_status_code(res.status())
        .into_response(res.request())?;
    
    Ok(ErrorHandlerResponse::Response(
        res.into_response(new_resp.into_body()),
    ))
}

pub fn error_handlers() -> ErrorHandlers<Body> {
    ErrorHandlers::new()
        .handler(http::StatusCode::INTERNAL_SERVER_ERROR, move |request| {
            render_error_page(request, "500")
        })
        .handler(http::StatusCode::BAD_REQUEST, move |request| {
            render_error_page(request, "400")
        })
        .handler(http::StatusCode::NOT_FOUND, move |request| {
            render_error_page(request, "404")
        })
}

pub fn config_error(_cfg: &mut ServiceConfig) {

}