package middlewares

import (
	"log"
	"net/http"
)

func LogMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		log.Println("Request Received 📦", r.Method, r.URL)
		next.ServeHTTP(w, r)
	})
}
