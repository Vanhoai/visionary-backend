package config

import (
	"strings"
	"sync"
	"time"

	"github.com/fsnotify/fsnotify"
	"github.com/spf13/cast"
	"github.com/spf13/viper"
)

type Env string

type Config struct {
	Env          Env             `yaml:"env" mapstructure:"env"`
	App          *AppConfig      `yaml:"app" mapstructure:"app"`
	Server       *ServerConfig   `yaml:"server" mapstructure:"server"`
	Metrics      *MetricsConfig  `yaml:"metrics" mapstructure:"metrics"`
	Log          *LogConfig      `yaml:"log" mapstructure:"log"`
	Redis        *RedisConfig    `yaml:"redis" mapstructure:"redis"`
	Postgres     *PostgresConfig `yaml:"postgres" mapstructure:"postgres"`
	Scylla       *ScyllaConfig   `yaml:"scylla" mapstructure:"scylla"`
	MigrationDir string          `yaml:"migration_dir" mapstructure:"migration_dir"`
}

type AppConfig struct {
	Name    string `yaml:"name" mapstructure:"name"`
	Debug   bool   `yaml:"debug" mapstructure:"debug"`
	Version string `yaml:"version" mapstructure:"version"`
}

type ServerConfig struct {
	Host         string `yaml:"host" mapstructure:"host"`
	Port         uint16 `yaml:"port" mapstructure:"port"`
	ReadTimeout  string `yaml:"read_timeout" mapstructure:"read_timeout"`
	WriteTimeout string `yaml:"write_timeout" mapstructure:"write_timeout"`
}

type MetricsConfig struct {
	Address string `yaml:"address" mapstructure:"address"`
	Enabled bool   `yaml:"enabled" mapstructure:"enabled"`
	Path    string `yaml:"path" mapstructure:"path"`
}

type LogConfig struct {
	SavePath         string `yaml:"save_path" mapstructure:"save_path"`
	FileName         string `yaml:"file_name" mapstructure:"file_name"`
	MaxSize          int    `yaml:"max_size" mapstructure:"max_size"`
	MaxAge           int    `yaml:"max_age" mapstructure:"max_age"`
	LocalTime        bool   `yaml:"local_time" mapstructure:"local_time"`
	Compress         bool   `yaml:"compress" mapstructure:"compress"`
	Level            string `yaml:"level" mapstructure:"level"`
	EnableConsole    bool   `yaml:"enable_console" mapstructure:"enable_console"`
	EnableColor      bool   `yaml:"enable_color" mapstructure:"enable_color"`
	EnableCaller     bool   `yaml:"enable_caller" mapstructure:"enable_caller"`
	EnableStacktrace bool   `yaml:"enable_stacktrace" mapstructure:"enable_stacktrace"`
}

type RedisConfig struct {
	Host         string `yaml:"host" mapstructure:"host"`
	Port         int    `yaml:"port" mapstructure:"port"`
	Password     string `yaml:"password" mapstructure:"password"`
	DB           int    `yaml:"db" mapstructure:"db"`
	PoolSize     int    `yaml:"pool_size" mapstructure:"pool_size"`
	MinIdleConns int    `yaml:"min_idle_conns" mapstructure:"min_idle_conns"`
	IdleTimeout  string `yaml:"idle_timeout" mapstructure:"idle_timeout"`
}

type PostgresConfig struct {
	User           string `yaml:"user" mapstructure:"user"`
	Password       string `yaml:"password" mapstructure:"password"`
	Host           string `yaml:"host" mapstructure:"host"`
	Port           int    `yaml:"port" mapstructure:"port"`
	Database       string `yaml:"database" mapstructure:"database"`
	SSLMode        string `yaml:"ssl_mode" mapstructure:"ssl_mode"`
	MaxConnections uint8  `yaml:"max_connections" mapstructure:"max_connections"`
	MinConnections uint8  `yaml:"min_connections" mapstructure:"min_connections"`
	ConnectTimeout uint8  `yaml:"connect_timeout" mapstructure:"connect_timeout"`
	TimeZone       string `yaml:"time_zone" mapstructure:"time_zone"`
}

type ScyllaConfig struct {
	Hosts             []string `yaml:"hosts" mapstructure:"hosts"`
	Port              uint16   `yaml:"port" mapstructure:"port"`
	Keyspace          string   `yaml:"keyspace" mapstructure:"keyspace"`
	Username          string   `yaml:"username" mapstructure:"username"`
	Password          string   `yaml:"password" mapstructure:"password"`
	Consistency       string   `yaml:"consistency" mapstructure:"consistency"`
	Timeout           string   `yaml:"timeout" mapstructure:"timeout"`
	MaxRetries        uint8    `yaml:"max_retries" mapstructure:"max_retries"`
	ReconnectInterval string   `yaml:"reconnect_interval" mapstructure:"reconnect_interval"`
	PoolSize          uint8    `yaml:"pool_size" mapstructure:"pool_size"`
}

var GlobalConfig *Config
var configMutex sync.RWMutex
var lastConfigChangeTime time.Time

// GetLastConfigChangeTime returns the time when the config was last changed
func GetLastConfigChangeTime() time.Time {
	configMutex.RLock()
	defer configMutex.RUnlock()
	return lastConfigChangeTime
}

func Load(configPath string, configFile string) (*Config, error) {
	var config *Config
	vip := viper.New()
	vip.AddConfigPath(configPath)
	vip.SetConfigName(configFile)
	vip.SetConfigType("yaml")

	if err := vip.ReadInConfig(); err != nil {
		return nil, err
	}

	// Enable environment variables to override config
	vip.SetEnvKeyReplacer(strings.NewReplacer(".", "_"))
	vip.AutomaticEnv()

	err := vip.Unmarshal(&config)
	if err != nil {
		return nil, err
	}

	applyEnvOverrides(config)

	vip.WatchConfig()
	vip.OnConfigChange(func(e fsnotify.Event) {
		var newConfig Config
		if err := vip.Unmarshal(&newConfig); err == nil {
			// Apply environment variable overrides to the new config
			applyEnvOverrides(&newConfig)

			// Update global config with new values - with mutex protection
			configMutex.Lock()
			*GlobalConfig = newConfig
			lastConfigChangeTime = time.Now()
			configMutex.Unlock()
		}
	})

	return config, nil
}

func applyEnvOverrides(config *Config) {}

func Init() {
	config, err := Load("./core/config", "config")
	if err != nil {
		panic("Load config fail : " + err.Error())
	}

	GlobalConfig = config
}

// GetDuration converts a duration string to time.Duration
func GetDuration(durationStr string) time.Duration {
	return cast.ToDuration(durationStr)
}
