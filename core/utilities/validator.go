package utilities

import (
	"errors"
	"fmt"
	"strings"
	"visionary-backend/core/https"
	"visionary-backend/core/safe"

	"github.com/go-playground/validator/v10"
)

var validate *validator.Validate

func InitValidator() {
	validate = validator.New()

	// Register custom validation functions or tag name functions here if needed
	safe.MustNoValue(validate.RegisterValidation("password", validatePassword))
}

func validatePassword(field validator.FieldLevel) bool {
	password := field.Field().String()
	// At least 8 characters, contains letter and number
	hasLetter := false
	hasNumber := false

	for _, char := range password {
		if char >= 'a' && char <= 'z' || char >= 'A' && char <= 'Z' {
			hasLetter = true
		}
		if char >= '0' && char <= '9' {
			hasNumber = true
		}
	}

	return len(password) >= 8 && hasLetter && hasNumber
}

func ValidateStruct(s interface{}) *https.AppError {
	err := validate.Struct(s)
	if err == nil {
		return nil
	}

	var validationErrors validator.ValidationErrors
	errors.As(err, &validationErrors)
	details := make(map[string]interface{})

	for _, fieldError := range validationErrors {
		fieldName := strings.ToLower(fieldError.Field())
		details[fieldName] = getErrorMessage(fieldError)
	}

	return https.NewValidationError("Validation failed", details)
}

func getErrorMessage(field validator.FieldError) string {
	switch field.Tag() {
	case "required":
		return fmt.Sprintf("%s is required", field.Field())
	case "email":
		return "Invalid email format"
	case "min":
		return fmt.Sprintf("%s must be at least %s characters", field.Field(), field.Param())
	case "max":
		return fmt.Sprintf("%s must be at most %s characters", field.Field(), field.Param())
	case "password":
		return "Password must be at least 8 characters and contain letters and numbers"
	default:
		return fmt.Sprintf("%s is invalid", field.Field())
	}
}
