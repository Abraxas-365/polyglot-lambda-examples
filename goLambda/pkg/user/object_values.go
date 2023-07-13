package user

type Name string

func (n Name) Validate() bool {
	if n == "" {
		return false
	}

	return true
}
