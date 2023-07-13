package pokemon

type Pokemon struct {
	Abilities      []AbilityInfo `json:"abilities"`
	BaseExperience int           `json:"base_experience"`
	Forms          []Form        `json:"forms"`
}

type Ability struct {
	Name string `json:"name"`
	URL  string `json:"url"`
}

type AbilityInfo struct {
	Ability  Ability `json:"ability"`
	IsHidden bool    `json:"is_hidden"`
	Slot     int     `json:"slot"`
}

type Form struct {
	Name string `json:"name"`
	URL  string `json:"url"`
}
