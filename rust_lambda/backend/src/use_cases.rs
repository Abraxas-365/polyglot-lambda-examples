use futures;
use lib::{
    errors::errors::Error,
    pokemon::{pokemon_repository::PokemonRepository, pokemon_type::Pokemon},
    user::{user_repository::UserRepository, user_type::User},
};
use rand::Rng;
use tokio;

pub struct UseCases {
    user_repo: Box<dyn UserRepository>,
    pokemos_repo: Box<dyn PokemonRepository>,
}

impl UseCases {
    pub fn new(
        user_repo: Box<dyn UserRepository>,
        pokemos_repo: Box<dyn PokemonRepository>,
    ) -> Self {
        Self {
            user_repo,
            pokemos_repo,
        }
    }

    pub async fn create_user(&self, user: User) -> Result<(), Error> {
        self.user_repo.save(user).await?;
        Ok(())
    }
    pub async fn get_user_pokemons(&self, user_id: &str) -> Result<Vec<Pokemon>, Error> {
        let pokemons_name = vec![
            "bulbasaur".to_string(),
            "ivysaur".to_string(),
            "venusaur".to_string(),
            "charmander".to_string(),
            "charmeleon".to_string(),
            "charizard".to_string(),
            "squirtle".to_string(),
            "wartortle".to_string(),
            "blastoise".to_string(),
            "caterpie".to_string(),
            "metapod".to_string(),
            "butterfree".to_string(),
            "weedle".to_string(),
            "kakuna".to_string(),
            "beedrill".to_string(),
            "pidgey".to_string(),
            "pidgeotto".to_string(),
            "pidgeot".to_string(),
            "rattata".to_string(),
            "raticate".to_string(),
        ];
        let user = self.user_repo.find_by_id(&user_id).await?;

        let mut user_pokemon_futures: Vec<_> = Vec::new();

        for _ in 0..user.number_of_pokeballs {
            let random_index = rand::thread_rng().gen_range(0..pokemons_name.len());
            let pokemon_future = self
                .pokemos_repo
                .find_by_name(pokemons_name[random_index].as_str());
            user_pokemon_futures.push(pokemon_future);
        }

        let user_pokemons: Result<Vec<_>, _> =
            futures::future::try_join_all(user_pokemon_futures).await;

        match user_pokemons {
            Ok(pokemons) => Ok(pokemons),
            Err(err) => Err(err),
        }
    }
}
