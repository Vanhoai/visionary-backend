package config

type ServerConfig struct {
	Host         string `env:"HOST" envDefault:"0.0.0.0"`
	Port         string `env:"PORT" envDefault:"8080"`
	ReadTimeout  int    `env:"READ_TIMEOUT" envDefault:"30"`
	WriteTimeout int    `env:"WRITE_TIMEOUT" envDefault:"30"`
}

type PostgresConfig struct {
	Host            string `env:"HOST"`
	Port            string `env:"PORT" envDefault:"5432"`
	Username        string `env:"USERNAME"`
	Password        string `env:"PASSWORD"`
	Database        string `env:"DATABASE"`
	MaxOpenConns    int    `env:"MAX_OPEN_CONNS" envDefault:"25"`
	MaxIdleConns    int    `env:"MAX_IDLE_CONNS" envDefault:"5"`
	ConnMaxLifetime int    `env:"CONN_MAX_LIFETIME" envDefault:"300"`
}

type ScyllaConfig struct {
	Hosts             []string `env:"HOSTS" envSeparator:","`
	Port              string   `env:"PORT" envDefault:"9042"`
	Keyspace          string   `env:"KEYSPACE"`
	Username          string   `env:"USERNAME"`
	Password          string   `env:"PASSWORD"`
	Consistency       string   `env:"CONSISTENCY" envDefault:"QUORUM"`
	Timeout           int      `env:"TIMEOUT" envDefault:"10"`
	ConnectTimeout    int      `env:"CONNECT_TIMEOUT" envDefault:"10"`
	NumConns          int      `env:"NUM_CONNS" envDefault:"2"`
	ReplicationFactor int      `env:"REPLICATION_FACTOR" envDefault:"3"`
}

type RedisConfig struct {
	Host         string `env:"HOST"`
	Port         string `env:"PORT" envDefault:"6379"`
	Password     string `env:"PASSWORD"`
	DB           string `env:"DB" envDefault:"0"`
	PoolSize     int    `env:"POOL_SIZE" envDefault:"10"`
	MinIdleConns int    `env:"MIN_IDLE_CONNS" envDefault:"5"`
	MaxRetries   int    `env:"MAX_RETRIES" envDefault:"3"`
	DialTimeout  int    `env:"DIAL_TIMEOUT" envDefault:"5"`
	ReadTimeout  int    `env:"READ_TIMEOUT" envDefault:"3"`
	WriteTimeout int    `env:"WRITE_TIMEOUT" envDefault:"3"`
}

type LoggingConfig struct {
	Level      string `env:"LEVEL" envDefault:"info"`
	Format     string `env:"FORMAT" envDefault:"json"`
	Output     string `env:"OUTPUT" envDefault:"stdout"`
	FilePath   string `env:"FILE_PATH" envDefault:"logs/app.log"`
	MaxSize    int    `env:"MAX_SIZE" envDefault:"100"`
	MaxBackups int    `env:"MAX_BACKUPS" envDefault:"3"`
	MaxAge     int    `env:"MAX_AGE" envDefault:"7"`
}
