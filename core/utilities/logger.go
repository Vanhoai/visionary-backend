package utilities

import "go.uber.org/zap"

// Global logger instances, maintained for compatibility with existing code
var (
	Logger        *zap.Logger
	SugaredLogger *zap.SugaredLogger
)

// LogContext holds contextual information for logging
type LogContext struct {
	RequestID string
	AccountID string
	TraceID   string
	SpanID    string
	Operation string
	Component string
}

func NewLogContext() *LogContext {
	return &LogContext{
		Component: "system",
	}
}

func (c *LogContext) WithComponent(component string) *LogContext {
	c.Component = component
	return c
}

func (c *LogContext) WithRequestID(requestID string) *LogContext {
	c.RequestID = requestID
	return c
}

func (c *LogContext) WithAccountID(accountID string) *LogContext {
	c.AccountID = accountID
	return c
}

func (c *LogContext) WithTraceID(traceID string) *LogContext {
	c.TraceID = traceID
	return c
}

func (c *LogContext) WithSpanID(spanID string) *LogContext {
	c.SpanID = spanID
	return c
}

func (c *LogContext) WithOperation(operation string) *LogContext {
	c.Operation = operation
	return c
}

func (c *LogContext) ToFields() []zap.Field {
	data := map[string]string{
		"request_id": c.RequestID,
		"account_id": c.AccountID,
		"trace_id":   c.TraceID,
		"span_id":    c.SpanID,
		"operation":  c.Operation,
		"component":  c.Component,
	}

	fields := make([]zap.Field, 0, len(data))
	for k, v := range data {
		if v != "" {
			fields = append(fields, zap.String(k, v))
		}
	}

	return fields
}
