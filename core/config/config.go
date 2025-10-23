package config

import (
	"strings"
	"sync"
	"time"
	"visionary-backend/core/safe"

	"github.com/fsnotify/fsnotify"
	"github.com/spf13/viper"
)

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
	return safe.Try(func() (*Config, error) {
		var config *Config
		vip := viper.New()
		vip.AddConfigPath(configPath)
		vip.SetConfigName(configFile)
		vip.SetConfigType("yaml")

		safe.MustNoValue(vip.ReadInConfig())

		// Enable environment variables to override config
		vip.SetEnvKeyReplacer(strings.NewReplacer(".", "_"))
		vip.AutomaticEnv()

		// Unmarshal config into struct
		safe.MustNoValue(vip.Unmarshal(&config))

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
	})
}

func applyEnvOverrides(config *Config) {}

func (c *Config) Validate() error {
	return nil
}

func Init() {
	config := safe.Must(Load("./core/config", "config"))
	safe.MustNoValue(config.Validate())

	GlobalConfig = config
}
