package middlewares

import (
	"log"
	"net/http"
)

func Cors(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		log.Println("CORS Middleware Executed 📦")
		next.ServeHTTP(w, r)
	})
}
