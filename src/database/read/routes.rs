use crate::models::database::Route;
use crate::database::DB_POOL;

use diesel::prelude::*;

pub fn get_all_routes() -> Vec<Route> {
    use crate::schema::routes::dsl::*;
    
    let connection = &*DB_POOL.get().unwrap();

    routes
        .filter(publication.eq(1))
        .load::<Route>(connection)
        .expect("Error loading all routes")
}