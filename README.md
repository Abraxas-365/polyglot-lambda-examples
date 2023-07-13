# polyglot-lambda-examples

This repository contains a collection of examples showcasing polyglot programming with Lambdas in Go, Rust, and Node.js. Explore various implementations and best practices for serverless functions across these languages. Learn how to harness the power of Lambdas in a multi-language environment through practical examples and code snippets.

# How to run and thest locally

You need to acces to aws in your env variables, and a dynamo table with the name of test_lambdas
Todo:: add the table name to env variable suport

## Go

Ensure you have SAM and Docker installed on your machine. If not, refer to the [official SAM documentation](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-sam-cli-install.html) for installation instructions.

Go to the go folder and run

```bash
go mod tidy
sam local start-api --host localhost --docker-network bridge
```

## Rust

Install cargo [[labda https://www.cargo-lambda.info/guide/installation.html#with-scoop-windows]]

Go to rust folder and rust

```bash
 cargo lambda watch #the first time will install dependencies could take a while
```

# What does the lambda does?

The lambda function is designed to handle user creation in a DynamoDB table. When invoked, it receives input parameters such as name, ID, and the number of pokeballs. It uses this information to create a new user entry in the DynamoDB table.
Furthermore, when you request information about a specific user, the lambda function leverages asynchronous programming to call the PokeAPI. By making an asynchronous call, the function can retrieve information about random pokemons based on the number of pokeballs associated with the user.

## Curl example to create a user

```curl
curl -X POST -H "Content-Type: application/json" -d '{
	"name": "test",
	"pokeball_number": 3
}' <lambda_endpoint_url>
```

## Curl example to get user

to know the user id go to the dynamo db

```curl
curl -X GET "http://localhost:9000?id=<user id>"
```
