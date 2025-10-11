module go-hexagonal-architecture

go 1.25.1

require (
	adapters v0.0.0
	core v0.0.0
	github.com/gofiber/fiber/v2 v2.52.9
)

require (
	github.com/andybalholm/brotli v1.2.0 // indirect
	github.com/caarlos0/env/v10 v10.0.0 // indirect
	github.com/clipperhouse/uax29/v2 v2.2.0 // indirect
	github.com/google/uuid v1.6.0 // indirect
	github.com/joho/godotenv v1.5.1 // indirect
	github.com/klauspost/compress v1.18.0 // indirect
	github.com/mattn/go-colorable v0.1.14 // indirect
	github.com/mattn/go-isatty v0.0.20 // indirect
	github.com/mattn/go-runewidth v0.0.19 // indirect
	github.com/valyala/bytebufferpool v1.0.0 // indirect
	github.com/valyala/fasthttp v1.66.0 // indirect
	golang.org/x/sys v0.37.0 // indirect
)

replace (
	adapters => ./adapters
	core => ./core
)
