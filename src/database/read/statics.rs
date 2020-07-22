use crate::models::database::Static;
use crate::database::DB_POOL;

use diesel::prelude::*;

pub fn get_statics_for_page(route: &str, page: &str) -> Vec<Static> {
    use crate::schema::pages;
    use crate::schema::statics;

    let connection = &*DB_POOL.get().unwrap();
}