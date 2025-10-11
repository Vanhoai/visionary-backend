package config

import (
	"fmt"
	"strings"

	"github.com/spf13/viper"
)

type Config struct {
	Environment string         `mapstructure:"environment"`
	Server      ServerConfig   `mapstructure:"server"`
	Postgres    PostgresConfig `mapstructure:"postgres"`
	Scylla      ScyllaConfig   `mapstructure:"scylla"`
	Redis       RedisConfig    `mapstructure:"redis"`
	Logging     LoggingConfig  `mapstructure:"logging"`
}

func LoadConfig(env string) (*Config, error) {
	viper.SetConfigFile(".env")
	viper.SetConfigType("env")

	// Read config file
	if err := viper.ReadInConfig(); err != nil {
		return nil, fmt.Errorf("error reading config file: %w", err)
	}

	// Environment variables override
	viper.SetEnvKeyReplacer(strings.NewReplacer(".", "_"))
	viper.AutomaticEnv()

	// Unmarshal config
	var config Config
	if err := viper.Unmarshal(&config); err != nil {
		return nil, fmt.Errorf("unable to decode config: %w", err)
	}

	// Override with environment variables if set
	bindEnvVariables()

	// Validate config
	if err := validateConfig(&config); err != nil {
		return nil, fmt.Errorf("config validation failed: %w", err)
	}

	return &config, nil
}

// bindEnvVariables binds environment variables to config keys
func bindEnvVariables() {
	// Environment
	viper.BindEnv("environment", "ENVIRONMENT")

	// Server
	viper.BindEnv("server.host", "SERVER_HOST")
	viper.BindEnv("server.port", "SERVER_PORT")
	viper.BindEnv("server.read_timeout", "SERVER_READ_TIMEOUT")
	viper.BindEnv("server.write_timeout", "SERVER_WRITE_TIMEOUT")

	// Postgres
	viper.BindEnv("postgres.host", "POSTGRES_HOST")
	viper.BindEnv("postgres.port", "POSTGRES_PORT")
	viper.BindEnv("postgres.username", "POSTGRES_USERNAME")
	viper.BindEnv("postgres.password", "POSTGRES_PASSWORD")
	viper.BindEnv("postgres.database", "POSTGRES_DATABASE")
	viper.BindEnv("postgres.max_open_conns", "POSTGRES_MAX_OPEN_CONNS")
	viper.BindEnv("postgres.max_idle_conns", "POSTGRES_MAX_IDLE_CONNS")
	viper.BindEnv("postgres.conn_max_lifetime", "POSTGRES_CONN_MAX_LIFETIME")

	// Scylla
	viper.BindEnv("scylla.hosts", "SCYLLA_HOSTS")
	viper.BindEnv("scylla.port", "SCYLLA_PORT")
	viper.BindEnv("scylla.keyspace", "SCYLLA_KEYSPACE")
	viper.BindEnv("scylla.username", "SCYLLA_USERNAME")
	viper.BindEnv("scylla.password", "SCYLLA_PASSWORD")
	viper.BindEnv("scylla.consistency", "SCYLLA_CONSISTENCY")
	viper.BindEnv("scylla.timeout", "SCYLLA_TIMEOUT")
	viper.BindEnv("scylla.connect_timeout", "SCYLLA_CONNECT_TIMEOUT")
	viper.BindEnv("scylla.num_conns", "SCYLLA_NUM_CONNS")
	viper.BindEnv("scylla.replication_factor", "SCYLLA_REPLICATION_FACTOR")

	// Redis
	viper.BindEnv("redis.host", "REDIS_HOST")
	viper.BindEnv("redis.port", "REDIS_PORT")
	viper.BindEnv("redis.password", "REDIS_PASSWORD")
	viper.BindEnv("redis.db", "REDIS_DATABASE")
	viper.BindEnv("redis.pool_size", "REDIS_POOL_SIZE")
	viper.BindEnv("redis.min_idle_conns", "REDIS_MIN_IDLE_CONNS")
	viper.BindEnv("redis.max_retries", "REDIS_MAX_RETRIES")
	viper.BindEnv("redis.dial_timeout", "REDIS_DIAL_TIMEOUT")
	viper.BindEnv("redis.read_timeout", "REDIS_READ_TIMEOUT")
	viper.BindEnv("redis.write_timeout", "REDIS_WRITE_TIMEOUT")

	// Logging
	viper.BindEnv("logging.level", "LOGGING_LEVEL")
	viper.BindEnv("logging.format", "LOGGING_FORMAT")
	viper.BindEnv("logging.output", "LOGGING_OUTPUT")
	viper.BindEnv("logging.file_path", "LOGGING_FILE_PATH")
	viper.BindEnv("logging.max_size", "LOGGING_MAX_SIZE")
	viper.BindEnv("logging.max_backups", "LOGGING_MAX_BACKUPS")
	viper.BindEnv("logging.max_age", "LOGGING_MAX_AGE")
}

// validateConfig validates the configuration
func validateConfig(config *Config) error {

	return nil
}
