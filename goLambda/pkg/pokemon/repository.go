package pokemon

type PokemonRepository interface {
	FindByName(name string) (Pokemon, error)
}
