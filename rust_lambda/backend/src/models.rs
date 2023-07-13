use serde::Serialize;
#[derive(Serialize)]
pub struct SuccessResponse {
    message: String,
}
#[derive(Serialize)]
pub enum LambdaResult {
    Pokemons(Vec<Pokemon>),
    Success(SuccessResponse),
}
