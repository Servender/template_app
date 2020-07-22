#[derive(Queryable, Clone)]
pub struct Route {
    pub id: i32,
    pub name: String,
    pub route: String,
    pub description: Option<String>,
    pub publication: i16
}

#[derive(Queryable)]
pub struct Page {
    pub id: i32,
    pub route_name: String,
    pub page_name: String,
    pub description: Option<String>,
    pub path: String
}

#[derive(Queryable)]
pub struct Static {
    pub id: i32,
    pub page_id: i32,
    pub name: Option<String>,
    pub type_file: String,
    pub status: i16,
    pub mask: String
}
