use async_trait::async_trait;

use crate::errors::errors::Error;

use super::pokemon_type::Pokemon;

#[async_trait]
pub trait PokemonRepository: Send + Sync {
    async fn find_by_name(&self, name: &str) -> Result<Pokemon, Error>;
}
