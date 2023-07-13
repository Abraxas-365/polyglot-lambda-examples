use async_trait::async_trait;
use reqwest::Client;

use crate::{
    errors::errors::Error,
    pokemon::{pokemon_repository::PokemonRepository, pokemon_type::Pokemon},
};

pub struct PokeApi;

impl PokeApi {
    pub fn new() -> Self {
        PokeApi
    }
}

#[async_trait]
impl PokemonRepository for PokeApi {
    async fn find_by_name(&self, name: &str) -> Result<Pokemon, Error> {
        let url = format!("https://pokeapi.co/api/v2/pokemon/{}", name);
        let client = Client::new();
        println!("VVVVVVVVVVVVVVVVV: {:?}", url);
        let resp = client
            .get(&url)
            .send()
            .await
            .map_err(|e| Error::InternalServerError(e.to_string()))?;
        println!("AAAAAAAAAA: {:?}", resp.status());

        match resp.status() {
            reqwest::StatusCode::OK => {
                let pokemon: Pokemon = resp
                    .json()
                    .await
                    .map_err(|e| Error::InternalServerError(e.to_string()))?;

                Ok(pokemon)
            }

            reqwest::StatusCode::NOT_FOUND => {
                Err(Error::NotFoundError(format!("Pokemon {} not found", name)))
            }
            reqwest::StatusCode::UNAUTHORIZED => Err(Error::UnauthorizedError(format!(
                "Unauthorized request to get Pokemon {}",
                name
            ))),
            reqwest::StatusCode::CONFLICT => Err(Error::ConflictError(format!(
                "Conflict with Pokemon {}",
                name
            ))),
            _ => Err(Error::InternalServerError(format!(
                "Failed to get Pokemon {}",
                name
            ))),
        }
    }
}
