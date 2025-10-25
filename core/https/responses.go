package https

import "time"

type Response[T any] struct {
	Code      string                 `json:"code,omitempty"`
	Message   string                 `json:"message"`
	Timestamp time.Time              `json:"timestamp"`
	Payload   T                      `json:"payload,omitempty"`
	Details   map[string]interface{} `json:"details,omitempty"`
}

func ResponseSuccess[T any](payload T, message string) *Response[T] {
	return &Response[T]{
		Code:      "SUCCESS",
		Message:   message,
		Payload:   payload,
		Timestamp: time.Now(),
	}
}

func NewErrorResponse(appError *AppError) *Response[any] {
	return &Response[any]{
		Code:      string(appError.Code),
		Message:   appError.Message,
		Details:   appError.Details,
		Timestamp: time.Now(),
	}
}

type Meta struct {
	Page        int   `json:"page"`
	PageSize    int   `json:"pageSize"`
	TotalRecord int64 `json:"totalRecord"`
	TotalPages  int   `json:"totalPages"`
}

type PaginatedResponse[T any] struct{}
