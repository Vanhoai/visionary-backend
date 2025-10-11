package config

import (
	"fmt"

	"github.com/caarlos0/env/v10"
	"github.com/joho/godotenv"
)

type Config struct {
	Environment string         `env:"ENVIRONMENT" envDefault:"development"`
	Server      ServerConfig   `envPrefix:"SERVER_"`
	Postgres    PostgresConfig `envPrefix:"POSTGRES_"`
	Scylla      ScyllaConfig   `envPrefix:"SCYLLA_"`
	Redis       RedisConfig    `envPrefix:"REDIS_"`
	Logging     LoggingConfig  `envPrefix:"LOGGING_"`
}

// LoadConfig loads configuration from environment variables
func LoadConfig() (*Config, error) {
	// Load .env file (optional, for local development)
	_ = godotenv.Load()

	cfg := &Config{}
	if err := env.Parse(cfg); err != nil {
		return nil, fmt.Errorf("failed to parse config: %w", err)
	}

	if err := validateConfig(cfg); err != nil {
		return nil, fmt.Errorf("invalid configuration: %w", err)
	}

	return cfg, nil
}

func validateConfig(cfg *Config) error {
	// Add any custom validation logic here if needed
	return nil
}
