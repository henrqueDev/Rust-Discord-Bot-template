use diesel::{prelude::{Insertable, Queryable}, Selectable};

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub  id: i32,
    pub name: String,
    pub id_discord: i64
}


#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct UserDTO {
    pub name: String,
    pub id_discord: i64
}