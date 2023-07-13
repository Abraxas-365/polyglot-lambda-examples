package errors

import "fmt"

type InternalServerError struct {
	Message string
}

func NewInternalServerError(message string) *InternalServerError {
	return &InternalServerError{Message: message}
}

func (e *InternalServerError) Error() string {
	return fmt.Sprintf("Internal server error: %s", e.Message)
}

type NotFoundError struct {
	Resource string
}

func NewNotFoundError(resource string) *NotFoundError {
	return &NotFoundError{Resource: resource}
}

func (e *NotFoundError) Error() string {
	return fmt.Sprintf("%s not found", e.Resource)
}

type UnauthorizedError struct {
	Operation string
}

func NewUnauthorizedError(operation string) *UnauthorizedError {
	return &UnauthorizedError{Operation: operation}
}

func (e *UnauthorizedError) Error() string {
	return fmt.Sprintf("Unauthorized: %s", e.Operation)
}

type ValidationError struct {
	Field   string
	Message string
}

func NewValidationError(field, message string) *ValidationError {
	return &ValidationError{Field: field, Message: message}
}

func (e *ValidationError) Error() string {
	return fmt.Sprintf("Validation error for field %s: %s", e.Field, e.Message)
}

type ConflictError struct {
	Resource string
}

func NewConflictError(resource string) *ConflictError {
	return &ConflictError{Resource: resource}
}

func (e *ConflictError) Error() string {
	return fmt.Sprintf("%s already exists", e.Resource)
}

type BadRequestError struct {
	Message string
}

func NewBadRequestError(message string) *BadRequestError {
	return &BadRequestError{Message: message}
}

func (e *BadRequestError) Error() string {
	return e.Message
}
