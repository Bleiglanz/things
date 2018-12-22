#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub username: String,
    pub remarks: String,
}
