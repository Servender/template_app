use crate::models::database::Page;
use crate::database::DB_POOL;

use diesel::prelude::*;

pub fn get_page(route: &str, page: &str) -> Page {
    use crate::schema::pages::dsl::*;
    
    let connection = &*DB_POOL.get().unwrap();

    pages
        .filter(
            route_name.eq(route)
            .and(page_name.eq(page))
        )
        .first::<Page>(connection)
        .expect(&format!(
            "Error loading page {page} for {route} routes",
            page=page,
            route=route
        ))
}