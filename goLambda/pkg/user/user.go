package user

import (
	"github.com/Abraxas-365/lambda-example/pkg/errors"
	"github.com/google/uuid"
)

type User struct {
	Id        string `json:"id" dynamodbav:"id"`
	Name      Name   `json:"name" dynamodbav:"name"`
	NPokeBall int    `json:"pokeball_number" dynamodbav:"pokeball_number"`
}

func New(name string, pockeBall int) (*User, error) {
	if ok := Name(name).Validate(); !ok {
		return nil, errors.NewValidationError("name", "Invalid name")
	}
	return &User{
		Id:        uuid.New().String(),
		Name:      Name(name),
		NPokeBall: pockeBall,
	}, nil
}

func (u *User) Validate() error {
	if ok := u.Name.Validate(); !ok {
		return errors.NewValidationError("name", "Invalid name")
	}
	return nil
}
