use crate::common::errors::Res;
use crate::data::infra::psql::TheClient;
use crate::domain::user::user_model::User;

pub struct UserDataStore {
    the_client: TheClient,
}

impl UserDataStore {
    pub fn new(the_client: TheClient) -> Self {
        Self { the_client }
    }

    //     pub async fn create_user(&self, user: &User) -> Res<super::user_entity::UserEntity> {
    //         let query = "insert into users (id, firstname, lastname, email, username, password, createdtime) values ($1, $2, $3, $4, $5, $6, $7);";
    //         let do_query = self.the_client.query_one(
    //             query,
    //             &[
    //                 &user.id.as_hyphenated(),
    //                 &user.first_name,
    //                 &user.last_name,
    //                 &user.email,
    //                 &user.username,
    //                 &user.password,
    //             ],
    //         ).await;
    //     }
    // }
}
