mod models;
mod use_cases;
use lambda_http::{http::Method, run, service_fn, Error, Request, RequestExt};
use lib::{
    middlewares::lambda_middleware::middleware_handle,
    pokemon::infrastructure::pokeapi_repo::PokeApi,
    user::{infrastructure::dynamo_repository::DynamoDbUserRepository, user_type::User},
};
use models::{LambdaResult, SuccessResponse};
use use_cases::UseCases;

async fn function_handler(event: Request) -> Result<LambdaResult, lib::errors::errors::Error> {
    let user_repository = DynamoDbUserRepository::new(String::from("test_lambdas")).await?;
    let pokemon_repository = PokeApi::new();
    let use_case = UseCases::new(Box::new(user_repository), Box::new(pokemon_repository));

    match event.method() {
        &Method::GET => {
            let id = event
                .query_string_parameters_ref()
                .and_then(|params| params.first("id"));
            match id {
                Some(id) => Ok(LambdaResult::Pokemons(
                    use_case.get_user_pokemons(id).await?,
                )),
                None => Err(lib::errors::errors::Error::ValidationError {
                    field: String::from("id"),
                    message: String::from("id is required"),
                }),
            }
        }

        &Method::POST => {
            let user = match serde_json::from_slice::<User>(event.body().as_ref()) {
                Ok(body) => User::new(body.name.to_string(), body.number_of_pokeballs)?,
                Err(_) => {
                    return Err(lib::errors::errors::Error::ValidationError {
                        field: String::from("body"),
                        message: String::from("invalid body"),
                    })
                }
            };
            use_case.create_user(user).await?;
            Ok(LambdaResult::Success(SuccessResponse {
                message: "User created successfully".to_string(),
            }))
        }

        _ => Err(lib::errors::errors::Error::ValidationError {
            field: String::from("method"),
            message: String::from("method not allowed"),
        }),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();
    let service = service_fn(|event| middleware_handle(event, function_handler));
    run(service).await
}
