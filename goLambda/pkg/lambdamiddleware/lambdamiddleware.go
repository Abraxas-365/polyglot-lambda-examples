package lambdamiddleware

import (
	"context"
	"encoding/json"
	"net/http"

	"github.com/Abraxas-365/lambda-example/pkg/errors"
	"github.com/aws/aws-lambda-go/events"
)

type RequestHandler func(ctx context.Context, event events.APIGatewayProxyRequest) (interface{}, error)

func ErrorHandlingMiddleware(handler RequestHandler) RequestHandler {
	return func(ctx context.Context, event events.APIGatewayProxyRequest) (interface{}, error) {
		resp, err := handler(ctx, event)
		if err != nil {
			httpError := errors.NewHTTPError(err)
			return createErrorResponse(httpError), nil
		}
		successBody, _ := json.Marshal(resp)
		return events.APIGatewayProxyResponse{
			StatusCode: http.StatusOK,
			Body:       string(successBody),
			Headers: map[string]string{
				"Content-Type": "application/json",
			},
		}, nil
	}
}

func createErrorResponse(httpError *errors.HTTPError) events.APIGatewayProxyResponse {
	errorBody, _ := json.Marshal(httpError)
	return events.APIGatewayProxyResponse{
		StatusCode: httpError.Code,
		Body:       string(errorBody),
		Headers: map[string]string{
			"Content-Type": "application/json",
		},
	}
}
