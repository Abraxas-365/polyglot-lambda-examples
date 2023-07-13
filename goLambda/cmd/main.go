package main

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/Abraxas-365/lambda-example/internal/usecases"
	"github.com/Abraxas-365/lambda-example/pkg/errors"
	"github.com/Abraxas-365/lambda-example/pkg/lambdamiddleware"
	"github.com/Abraxas-365/lambda-example/pkg/logger"
	"github.com/Abraxas-365/lambda-example/pkg/pokemon/infrastructure/pokeapirepo"
	"github.com/Abraxas-365/lambda-example/pkg/user"
	"github.com/Abraxas-365/lambda-example/pkg/user/infrastructure/userdynamorepo"
	"github.com/aws/aws-lambda-go/events"
	"github.com/aws/aws-lambda-go/lambda"
	"github.com/aws/aws-lambda-go/lambdacontext"
	"github.com/aws/aws-sdk-go/aws"
	"github.com/aws/aws-sdk-go/aws/session"
	"github.com/aws/aws-sdk-go/service/dynamodb"
)

func handleRequest(ctx context.Context, event events.APIGatewayProxyRequest) (interface{}, error) {
	lambdaCtx, ok := lambdacontext.FromContext(ctx)
	if !ok {
		err := errors.NewInternalServerError("Error getting lambda context")
		logger.Error(lambdaCtx, err)
		return nil, err
	}

	sess, err := session.NewSession(&aws.Config{
		Region: aws.String("us-east-1"),
	})
	if err != nil {
		err := errors.NewInternalServerError("Error creating session")
		logger.Error(lambdaCtx, err)
		return nil, err
	}

	userRepository := userdynamorepo.New(dynamodb.New(sess), "test_lambdas")
	pokemonRepository := pokeapirepo.New()

	service := usecases.New(userRepository, pokemonRepository)
	switch event.HTTPMethod {
	case "GET":
		if event.QueryStringParameters["id"] == "" {
			return nil, errors.NewBadRequestError("id is required")
		}
		pokemons, err := service.GetUserPokemons(event.QueryStringParameters["id"])
		if err != nil {
			return nil, err
		}

		fmt.Println("BBBBBBBBBBBBBBBBBBB", pokemons)
		return pokemons, nil

	case "POST":
		var newUser user.User
		err := json.Unmarshal([]byte(event.Body), &newUser)
		if err != nil {
			return nil, errors.NewBadRequestError("Invalid request body")
		}

		u, err := user.New(string(newUser.Name), newUser.NPokeBall)
		if err != nil {
			return nil, err
		}

		// Save the user to the repository
		err = service.CreateUser(*u)
		if err != nil {
			return nil, err
		}

		return u, nil
	}

	return nil, errors.NewBadRequestError("Method not allowed")
}

func main() {
	lambda.Start(lambdamiddleware.ErrorHandlingMiddleware(handleRequest))
}
