package https

import "fmt"

type ErrorCode string

const (
	BadRequest         ErrorCode = "BAD_REQUEST"
	Unauthorized       ErrorCode = "UNAUTHORIZED"
	Forbidden          ErrorCode = "FORBIDDEN"
	NotFound           ErrorCode = "NOT_FOUND"
	Conflict           ErrorCode = "CONFLICT"
	Validation         ErrorCode = "VALIDATION_ERROR"
	InternalServer     ErrorCode = "INTERNAL_SERVER_ERROR"
	ServiceUnavailable ErrorCode = "SERVICE_UNAVAILABLE"
)

// AppError represents application-level errors
type AppError struct {
	Code       ErrorCode      `json:"code"`
	Message    string         `json:"message"`
	Details    map[string]any `json:"details,omitempty"`
	StatusCode int            `json:"statusCode"`
}

// Error implements the error interface
func (e *AppError) Error() string {
	return fmt.Sprintf("[%s] %s", e.Code, e.Message)
}

// Error constructors
func NewBadRequestError(message string, details map[string]any) *AppError {
	return &AppError{
		Code:       BadRequest,
		Message:    message,
		Details:    details,
		StatusCode: 400,
	}
}

func NewUnauthorizedError(message string, details map[string]any) *AppError {
	return &AppError{
		Code:       Unauthorized,
		Message:    message,
		Details:    details,
		StatusCode: 401,
	}
}

func NewForbiddenError(message string, details map[string]any) *AppError {
	return &AppError{
		Code:       Forbidden,
		Message:    message,
		Details:    details,
		StatusCode: 403,
	}
}

func NewNotFoundError(message string, details map[string]any) *AppError {
	return &AppError{
		Code:       NotFound,
		Message:    message,
		Details:    details,
		StatusCode: 404,
	}
}

func NewConflictError(message string, details map[string]any) *AppError {
	return &AppError{
		Code:       Conflict,
		Message:    message,
		Details:    details,
		StatusCode: 409,
	}
}

func NewValidationError(message string, details map[string]any) *AppError {
	return &AppError{
		Code:       Validation,
		Message:    message,
		Details:    details,
		StatusCode: 422,
	}
}

func NewInternalServerError(message string, details map[string]any) *AppError {
	return &AppError{
		Code:       InternalServer,
		Message:    message,
		Details:    details,
		StatusCode: 500,
	}
}
