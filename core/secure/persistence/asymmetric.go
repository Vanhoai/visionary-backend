package persistence

import (
	"crypto/rand"
	"crypto/rsa"
	"crypto/x509"
	"encoding/pem"
	"fmt"
	"log"
	"os"
)

// AsymmetricPersistence represents the storage structure for asymmetric cryptographic keys.
// Functions:
// LoadKeys: Loads the private and public keys from specified file paths.
// GenerateAndSave: Generates a new pair of asymmetric keys and saves them to specified file paths.

type AsymmetricPersistence struct {
	PrivateKey []byte
	PublicKey  []byte
}

func GenerateRSA(bits int) (*rsa.PrivateKey, *rsa.PublicKey) {
	privateKey, err := rsa.GenerateKey(rand.Reader, bits)
	if err != nil {
		log.Fatalf("Error generating RSA private key: %v", err)
	}

	return privateKey, &privateKey.PublicKey
}

func GenerateAndSaveRSAKeys() {
	privateKey, publicKey := GenerateRSA(2048)

	privatePem := pem.EncodeToMemory(&pem.Block{
		Type:  "RSA PRIVATE KEY",
		Bytes: x509.MarshalPKCS1PrivateKey(privateKey),
	})

	publicASN1, _ := x509.MarshalPKIXPublicKey(publicKey)
	publicPem := pem.EncodeToMemory(&pem.Block{
		Type:  "RSA PUBLIC KEY",
		Bytes: publicASN1,
	})

	// save to files
	err := os.WriteFile("keys/private_key.pem", privatePem, 0600)
	if err != nil {
		log.Fatalf("Error writing private key to file: %v", err)
	}

	err = os.WriteFile("keys/public_key.pem", publicPem, 0644)
	if err != nil {
		log.Fatalf("Error writing public key to file: %v", err)
	}

	fmt.Println("RSA keys generated and saved to keys/private_key.pem and keys/public_key.pem")
}

func LoadRSAPrivateKey(path string) (*rsa.PrivateKey, error) {
	privPem, err := os.ReadFile(path)
	if err != nil {
		return nil, err
	}

	block, _ := pem.Decode(privPem)
	if block == nil || block.Type != "RSA PRIVATE KEY" {
		return nil, fmt.Errorf("failed to decode PEM block containing private key")
	}

	privateKey, err := x509.ParsePKCS1PrivateKey(block.Bytes)
	if err != nil {
		return nil, err
	}

	return privateKey, nil
}

func LoadRSAPublicKey(path string) (*rsa.PublicKey, error) {
	pubPem, err := os.ReadFile(path)
	if err != nil {
		return nil, err
	}

	block, _ := pem.Decode(pubPem)
	if block == nil || block.Type != "RSA PUBLIC KEY" {
		return nil, fmt.Errorf("failed to decode PEM block containing public key")
	}

	pubInterface, err := x509.ParsePKIXPublicKey(block.Bytes)
	if err != nil {
		return nil, err
	}

	publicKey, ok := pubInterface.(*rsa.PublicKey)
	if !ok {
		return nil, fmt.Errorf("not RSA public key")
	}

	return publicKey, nil
}

func EncodeAndDecode(message string) {
	privateKey, err := LoadRSAPrivateKey("keys/private_key.pem")
	if err != nil {
		log.Fatalf("Error loading private key: %v", err)
	}

	publicKey, err := LoadRSAPublicKey("keys/public_key.pem")
	if err != nil {
		log.Fatalf("Error loading public key: %v", err)
	}

	// Encrypt the message
	ciphertext, err := rsa.EncryptPKCS1v15(rand.Reader, publicKey, []byte(message))
	if err != nil {
		log.Fatalf("Error encrypting message: %v", err)
	}

	fmt.Printf("Encrypted message: %x\n", ciphertext)

	// Decrypt the message
	plaintext, err := rsa.DecryptPKCS1v15(rand.Reader, privateKey, ciphertext)
	if err != nil {
		log.Fatalf("Error decrypting message: %v", err)
	}

	fmt.Printf("Decrypted message: %s\n", string(plaintext))
}
