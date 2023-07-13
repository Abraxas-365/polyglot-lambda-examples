use crate::{
    errors::errors::Error,
    user::{user_repository::UserRepository, user_type::User},
};

use async_trait::async_trait;
use aws_sdk_dynamodb::{types::AttributeValue, Client};

pub struct DynamoDbUserRepository {
    client: Client,
    table: String,
}

impl DynamoDbUserRepository {
    pub async fn new(table: String) -> Result<Self, Error> {
        let shared_config = aws_config::load_from_env().await;

        let client = Client::new(&shared_config);

        Ok(DynamoDbUserRepository { client, table })
    }
}

#[async_trait]
impl UserRepository for DynamoDbUserRepository {
    async fn save(&self, user: User) -> Result<(), Error> {
        let user_av = AttributeValue::S(user.id.clone());
        let name_av = AttributeValue::S(user.name.to_string());
        let pokeballs_av = AttributeValue::N(user.number_of_pokeballs.to_string());

        let put_item = self
            .client
            .put_item()
            .table_name(self.table.clone())
            .item("id", user_av)
            .item("name", name_av)
            .item("pokeball_number", pokeballs_av)
            .send()
            .await;

        match put_item {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::internal_server_error(e.to_string())),
        }
    }

    async fn find_by_id(&self, id: &str) -> Result<User, Error> {
        let get_item_output = self
            .client
            .get_item()
            .table_name(self.table.clone())
            .key("id", AttributeValue::S(id.to_string()))
            .send()
            .await
            .map_err(|e| {
                Error::internal_server_error(format!(
                    "Error getting user from DynamoDB: {}",
                    e.to_string()
                ))
            })?;

        match get_item_output.item {
            Some(item) => {
                let user: User = serde_dynamo::from_item(item).map_err(|e| {
                    Error::internal_server_error(format!(
                        "Error deserializing user from DynamoDB: {}",
                        e.to_string()
                    ))
                })?;
                Ok(user)
            }
            error => Err(Error::internal_server_error(format!(
                "Error getting user from DynamoDB: {:?}",
                error
            ))),
        }
    }
}
