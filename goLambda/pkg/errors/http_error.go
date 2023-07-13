package errors

import "net/http"

type HTTPError struct {
	Code    int    `json:"code"`
	Message string `json:"message"`
}

// NewHTTPError maps an application error to an HTTP error
func NewHTTPError(err error) *HTTPError {
	switch e := err.(type) {
	case *NotFoundError:
		return &HTTPError{Code: http.StatusNotFound, Message: e.Error()}
	case *UnauthorizedError:
		return &HTTPError{Code: http.StatusUnauthorized, Message: e.Error()}
	case *ValidationError:
		return &HTTPError{Code: http.StatusBadRequest, Message: e.Error()}
	case *ConflictError:
		return &HTTPError{Code: http.StatusConflict, Message: e.Error()}
	case *BadRequestError:
		return &HTTPError{Code: http.StatusBadRequest, Message: e.Error()}
	default:
		return &HTTPError{Code: http.StatusInternalServerError, Message: "Internal server error"}
	}
}
