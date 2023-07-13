package userdynamorepo

import (
	"github.com/Abraxas-365/lambda-example/pkg/errors"
	"github.com/Abraxas-365/lambda-example/pkg/user"
	"github.com/aws/aws-sdk-go/aws"
	"github.com/aws/aws-sdk-go/service/dynamodb"
	"github.com/aws/aws-sdk-go/service/dynamodb/dynamodbattribute"
)

type DynamoDBUserRepository struct {
	db        *dynamodb.DynamoDB
	tableName string
}

func New(db *dynamodb.DynamoDB, tableName string) user.UserRepository {
	return &DynamoDBUserRepository{db: db, tableName: tableName}
}

func (r *DynamoDBUserRepository) Save(user user.User) error {
	av, err := dynamodbattribute.MarshalMap(user)
	if err != nil {
		return errors.NewInternalServerError(err.Error())
	}

	input := &dynamodb.PutItemInput{
		Item:      av,
		TableName: aws.String(r.tableName),
	}

	_, err = r.db.PutItem(input)
	if err != nil {
		return errors.NewInternalServerError(err.Error())
	}

	return nil
}

func (r *DynamoDBUserRepository) FindById(id string) (user.User, error) {
	result, err := r.db.GetItem(&dynamodb.GetItemInput{
		TableName: aws.String(r.tableName),
		Key: map[string]*dynamodb.AttributeValue{
			"id": {
				S: aws.String(id),
			},
		},
	})

	if err != nil {
		return user.User{}, errors.NewInternalServerError(err.Error())
	}

	if result.Item == nil {
		return user.User{}, errors.NewNotFoundError("User")
	}

	var u user.User
	err = dynamodbattribute.UnmarshalMap(result.Item, &u)
	if err != nil {
		return user.User{}, errors.NewInternalServerError(err.Error())
	}

	return u, nil
}
