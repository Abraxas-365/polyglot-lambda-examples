package usecases

import (
	"fmt"
	"math/rand"
	"sync"

	"github.com/Abraxas-365/lambda-example/pkg/pokemon"
	"github.com/Abraxas-365/lambda-example/pkg/user"
)

type Usecases struct {
	userRepo    user.UserRepository
	pokemonRepo pokemon.PokemonRepository
}

func New(userRepo user.UserRepository, pokemonRepo pokemon.PokemonRepository) *Usecases {
	return &Usecases{
		userRepo:    userRepo,
		pokemonRepo: pokemonRepo,
	}
}

func (u *Usecases) CreateUser(user user.User) error {
	return u.userRepo.Save(user)
}

func (u *Usecases) GetUserPokemons(userId string) ([]pokemon.Pokemon, error) {
	pokemonList := []string{
		"bulbasaur", "ivysaur", "venusaur", "charmander",
		"charmeleon", "charizard", "squirtle", "wartortle",
		"blastoise", "caterpie", "metapod", "butterfree",
		"weedle", "kakuna", "beedrill", "pidgey", "pidgeotto",
		"pidgeot", "rattata", "raticate",
	}

	user, err := u.userRepo.FindById(userId)
	if err != nil {
		return nil, err
	}
	fmt.Println(user)

	var wg sync.WaitGroup

	resultCh := make(chan pokemon.Pokemon, user.NPokeBall)
	errorCh := make(chan error, user.NPokeBall)

	for i := 0; i < user.NPokeBall; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()

			randomIndex := rand.Intn(len(pokemonList))

			pokemonName := pokemonList[randomIndex]
			pokemon, err := u.pokemonRepo.FindByName(pokemonName)
			if err != nil {
				errorCh <- err
				return
			}
			resultCh <- pokemon
		}()
	}

	go func() {
		wg.Wait()
		close(resultCh)
		close(errorCh)
	}()

	select {
	case err := <-errorCh:
		return nil, err
	default:
	}

	var pokemons []pokemon.Pokemon
	for i := 0; i < user.NPokeBall; i++ {
		pokemon := <-resultCh
		pokemons = append(pokemons, pokemon)
	}

	return pokemons, nil
}
