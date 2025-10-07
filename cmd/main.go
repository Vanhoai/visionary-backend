package main

import (
	"fmt"
	"log"
	"reflect"
	"server/adapters/primary/middlewares"
	"server/adapters/secondary/repositories/scylla"
	"sync"

	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/middleware/adaptor"
)

type Container struct {
	mutex     sync.RWMutex             // Protects the services and providers maps in concurrent scenarios
	services  map[string]interface{}   // Holds instantiated services
	providers map[string]reflect.Value // Holds provider functions for services
}

func CreateNewContainer() *Container {
	return &Container{
		services:  make(map[string]interface{}),
		providers: make(map[string]reflect.Value),
	}
}

// Register a service with the factory function
func (c *Container) Register(name string, provider interface{}) error {
	c.mutex.Lock()
	defer c.mutex.Unlock()

	var providerValue = reflect.ValueOf(provider)
	if providerValue.Kind() != reflect.Func {
		return fmt.Errorf("provider must be a function")
	}

	c.providers[name] = providerValue
	fmt.Printf("Registered service: %s\n", name)
	return nil
}

// Register singleton service
func (c *Container) Singleton(provider interface{}) error {
	// Retrieve name from struct return type
	providerValue := reflect.ValueOf(provider)
	if providerValue.Kind() != reflect.Func {
		return fmt.Errorf("provider must be a function")
	}

	providerType := providerValue.Type()
	if providerType.NumOut() == 0 {
		return fmt.Errorf("provider must return at least one value")
	}

	returnsType := providerType.Out(0)
	var name string
	if returnsType.Kind() == reflect.Ptr {
		name = returnsType.Elem().Name()
	} else {
		name = returnsType.Name()
	}

	return c.Register(name, provider)
}

// Get retrieve a service by name, instantiating it if necessary
func (c *Container) Get(name string) (interface{}, error) {
	c.mutex.RLock()

	// check if service is already instantiated
	if service, exists := c.services[name]; exists {
		c.mutex.RUnlock()
		return service, nil
	}

	// check if provider exists
	provider, exists := c.providers[name]
	c.mutex.RUnlock()

	if !exists {
		return nil, fmt.Errorf("service not found: %s", name)
	}

	// Resolve dependencies and call provider
	args, err := c.resolveProviderArgs(provider)
	if err != nil {
		return nil, fmt.Errorf("failed to resolve dependencies for %s: %w", name, err)
	}

	results := provider.Call(args)
	if len(results) == 0 {
		return nil, fmt.Errorf("provider must return at least one value")
	}

	service := results[0].Interface()

	// Now acquire write lock to store the result
	c.mutex.Lock()
	defer c.mutex.Unlock()

	// Double-check if another goroutine already created it
	if existingService, exists := c.services[name]; exists {
		return existingService, nil
	}

	c.services[name] = service
	return service, nil
}

func (c *Container) resolveProviderArgs(provider reflect.Value) ([]reflect.Value, error) {

	providerType := provider.Type()
	numArgs := providerType.NumIn()
	args := make([]reflect.Value, numArgs)

	for i := 0; i < numArgs; i++ {
		argType := providerType.In(i)

		// Try to find a registered service that matches the argument type
		var serviceName string
		if argType.Kind() == reflect.Ptr {
			serviceName = argType.Elem().Name()
		} else {
			serviceName = argType.Name()
		}

		service, err := c.Get(serviceName)
		if err != nil {
			return nil, fmt.Errorf("failed to resolve dependency for %s: %w", serviceName, err)
		}

		args[i] = reflect.ValueOf(service)
	}

	return args, nil
}

// Make creates an instance of a struct and auto-injects dependencies using `inject` tags
func (c *Container) Make(target interface{}) error {
	targetValue := reflect.ValueOf(target)
	if targetValue.Kind() != reflect.Ptr {
		return fmt.Errorf("target must be a pointer")
	}

	targetValue = targetValue.Elem()
	if targetValue.Kind() != reflect.Struct {
		return fmt.Errorf("target must be a pointer to struct")
	}

	targetType := targetValue.Type()
	for i := 0; i < targetValue.NumField(); i++ {
		field := targetValue.Field(i)
		fieldType := targetType.Field(i)

		// Check for inject tag
		injectTag := fieldType.Tag.Get("inject")
		if injectTag == "" {
			continue
		}

		service, err := c.Get(injectTag)
		if err != nil {
			return fmt.Errorf("failed to inject dependency for field %s: %v", fieldType.Name, err)
		}

		if field.Kind() != reflect.Ptr {
			return fmt.Errorf("field %s is not a pointer", fieldType.Name)
		}

		field.Set(reflect.ValueOf(service))
	}

	return nil
}

func main() {
	di := CreateNewContainer()
	di.Singleton(scylla.NewAccountRepository)

	app := fiber.New()
	app.Use(adaptor.HTTPMiddleware(middlewares.LogMiddleware))

	log.Fatal(app.Listen(":3000"))
}
