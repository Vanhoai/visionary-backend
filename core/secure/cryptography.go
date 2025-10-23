package secure

// Environment variables for cryptography
// CRYPTOGRAPHY_TYPE = SYMMETRIC | ASYMMETRIC
// ALGORITHM = RSA | EC | HS512

type CryptoType string
type CryptoAlgorithm string

const (
	SYMMETRIC  CryptoType = "SYMMETRIC"
	ASYMMETRIC CryptoType = "ASYMMETRIC"
)

const (
	RSA CryptoAlgorithm = "RSA"
	EC  CryptoAlgorithm = "EC"
)

type Cryptography struct{}

func (c *Cryptography) GetEncodeKey() ([]byte, error) {
	return []byte("your-encode-key"), nil
}

func (c *Cryptography) GetDecodeKey() ([]byte, error) {
	return []byte("your-decode-key"), nil
}
