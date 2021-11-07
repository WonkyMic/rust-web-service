use crate::schema::users;
#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub handle: String
}