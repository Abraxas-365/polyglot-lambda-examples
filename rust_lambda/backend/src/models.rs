use lib::pokemon::pokemon_type::Pokemon;
use serde::Serialize;
#[derive(Serialize)]
pub struct SuccessResponse {
    pub message: String,
}
#[derive(Serialize)]
pub enum LambdaResult {
    Pokemons(Vec<Pokemon>),
    Success(SuccessResponse),
}
