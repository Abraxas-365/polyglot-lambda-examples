package logger

import (
	"encoding/json"
	"github.com/aws/aws-lambda-go/lambdacontext"
	"log"
)

type CloudWatchLog struct {
	RequestId string      `json:"requestId"`
	Message   string      `json:"message"`
	Error     interface{} `json:"error"`
}

func Ok(ctx *lambdacontext.LambdaContext, msg string) {
	logMessage := CloudWatchLog{
		RequestId: ctx.AwsRequestID,
		Message:   msg,
		Error:     nil,
	}

	logJSON, _ := json.Marshal(logMessage)
	log.Println(string(logJSON))
}

func Error(ctx *lambdacontext.LambdaContext, err error) {
	logMessage := CloudWatchLog{
		RequestId: ctx.AwsRequestID,
		Message:   "Error",
		Error:     err.Error(),
	}

	logJSON, _ := json.Marshal(logMessage)
	log.Println(string(logJSON))
}
