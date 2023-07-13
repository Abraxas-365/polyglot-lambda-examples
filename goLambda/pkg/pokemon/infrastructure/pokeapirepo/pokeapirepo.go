package pokeapirepo

import (
	"encoding/json"
	"fmt"
	"net/http"

	"github.com/Abraxas-365/lambda-example/pkg/errors"
	"github.com/Abraxas-365/lambda-example/pkg/pokemon"
)

type httpRepository struct{}

func New() pokemon.PokemonRepository {
	return &httpRepository{}
}

func (r httpRepository) FindByName(name string) (pokemon.Pokemon, error) {
	url := fmt.Sprintf("https://pokeapi.co/api/v2/pokemon/%s", name)
	resp, err := http.Get(url)
	if err != nil {
		return pokemon.Pokemon{}, errors.NewInternalServerError("Error getting pokemon")
	}
	defer resp.Body.Close()

	var p pokemon.Pokemon
	err = json.NewDecoder(resp.Body).Decode(&p)
	if err != nil {
		return pokemon.Pokemon{}, errors.NewInternalServerError("Error decoding pokemon")
	}

	return p, nil
}
