package secure

// Env:
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

type Cryptography interface {
	GetEncodeKey() ([]byte, error)
	GetDecodeKey() ([]byte, error)
}
