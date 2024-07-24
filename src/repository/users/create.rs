use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::model::user::User;
use crate::{model::user::UserDTO, repository::connection_handler::get_connection};

use crate::schema::users::dsl::*;



pub async fn create(user_name: String, discord_id: i64) {
    let connection = &mut get_connection();

    let user_name = String::from(user_name);

    let obj = UserDTO { name: user_name, id_discord: discord_id };
    
    let exist_user = users
        .filter(id_discord.eq(discord_id))
        .select(User::as_select())
        .first(connection);

    if exist_user.is_ok() {
        format!("User already verified");
        return;
    }

    diesel::insert_into(users)
        .values(&obj)
        .returning(User::as_returning())
        .get_result( connection)
        .expect("Error saving new post");

    format!("Sucess!");
}