package di

import (
	"fmt"
	"reflect"
	"sync"
	"visionary-backend/core/safe"
)

type Lifecycle uint8

const (
	Singleton Lifecycle = iota // Single instance shared across all resolutions
	Transient                  // New instance created for each resolution
	Scoped                     // Single instance per scope (future implementation)
)

type ServiceDescriptor struct {
	Name      string        // Name of the service
	Lifecycle Lifecycle     // Lifecycle of the service
	Provider  reflect.Value // Provider function for creating the service
	Instance  any           // Cached instance for Singleton services
	Type      reflect.Type  // Type of the service
}

type DIC struct {
	mutex           sync.RWMutex
	services        map[string]*ServiceDescriptor
	resolutionStack []string // For detecting circular dependencies
	resolving       sync.Map // Track services currently being resolved
}

func NewDIC() *DIC {
	return &DIC{
		services:        make(map[string]*ServiceDescriptor),
		resolutionStack: make([]string, 0),
	}
}

// DIC Register registers a service with the given name, provider function, and lifecycle
func (di *DIC) Register(name string, provider any, lifecycle Lifecycle) error {
	di.mutex.Lock()
	defer di.mutex.Unlock()

	providerValue := reflect.ValueOf(provider)
	safe.Assert(providerValue.Kind() == reflect.Func, fmt.Sprintf("provider for '%s' must be a function", name))

	providerType := providerValue.Type()
	safe.Assert(providerType.NumOut() == 1, fmt.Sprintf("provider for '%s' must return exactly one value", name))

	returnType := providerType.Out(0)
	di.services[name] = &ServiceDescriptor{
		Name:      name,
		Lifecycle: lifecycle,
		Provider:  providerValue,
		Type:      returnType,
	}

	return nil
}

// DIC RegisterInstance registers an existing instance as a singleton service
func (di *DIC) RegisterInstance(name string, instance any) error {
	di.mutex.Lock()
	defer di.mutex.Unlock()

	instanceValue := reflect.ValueOf(instance)

	di.services[name] = &ServiceDescriptor{
		Name:      name,
		Lifecycle: Singleton,
		Instance:  instance,
		Type:      instanceValue.Type(),
	}

	return nil
}

// Singleton registers a struct as a singleton with auto-injection
func (di *DIC) Singleton(name string, provider any) error {
	providerValue := reflect.ValueOf(provider)

	// If it's already a function, register directly as singleton
	if providerValue.Kind() == reflect.Func {
		return di.Register(name, provider, Singleton)
	}

	// If it's a struct, create a factory function with auto-injection
	safe.Assert(providerValue.Kind() == reflect.Struct, fmt.Sprintf("provider for '%s' must be a struct or function", name))

	providerType := providerValue.Type()
	factory := func() any {
		return di.createInstanceWithInjection(providerType)
	}

	return di.Register(name, factory, Singleton)
}

// createInstanceWithInjection creates a new instance with auto-injected dependencies
func (c *DIC) createInstanceWithInjection(structType reflect.Type) any {
	instance := reflect.New(structType).Elem()

	// Inject dependencies based on struct field tags
	for i := range structType.NumField() {
		field := structType.Field(i)
		fieldValue := instance.Field(i)

		if !fieldValue.CanSet() {
			continue
		}

		injectTag := field.Tag.Get("inject")
		if injectTag == "" {
			continue
		}

		dependency := safe.Must(c.Resolve(injectTag))

		// Type checking
		dependencyValue := reflect.ValueOf(dependency)
		safe.Assert(
			dependencyValue.Type().AssignableTo(fieldValue.Type()),
			fmt.Sprintf("dependency '%s' type mismatch: expected %s, got %s",
				injectTag, fieldValue.Type(), dependencyValue.Type()),
		)

		fieldValue.Set(dependencyValue)
	}

	return instance.Addr().Interface()
}

// Resolve resolves a service by name
func (c *DIC) Resolve(name string) (any, error) {
	return safe.Try(func() (any, error) {
		// Check for circular dependencies
		if _, isResolving := c.resolving.Load(name); isResolving {
			return nil, fmt.Errorf("circular dependency detected while resolving '%s': %v", name, c.resolutionStack)
		}

		c.resolving.Store(name, true)
		c.resolutionStack = append(c.resolutionStack, name)

		defer func() {
			c.resolving.Delete(name)
			if len(c.resolutionStack) > 0 {
				c.resolutionStack = c.resolutionStack[:len(c.resolutionStack)-1]
			}
		}()

		c.mutex.RLock()
		descriptor, exists := c.services[name]
		c.mutex.RUnlock()

		safe.Assert(exists, fmt.Sprintf("no provider registered for '%s'", name))

		// For Singleton, return cached instance if exists
		if descriptor.Lifecycle == Singleton && descriptor.Instance != nil {
			return descriptor.Instance, nil
		}

		// Call provider to create instance
		results := descriptor.Provider.Call([]reflect.Value{})
		safe.Assert(len(results) >= 1, fmt.Sprintf("provider for '%s' must return at least one value", name))

		service := results[0].Interface()

		// Cache singleton instances
		if descriptor.Lifecycle == Singleton {
			c.mutex.Lock()
			descriptor.Instance = service
			c.mutex.Unlock()
		}

		return service, nil
	})
}

// ResolveTyped resolves a service by name with type safety using generics
func ResolveTyped[T any](c *DIC, name string) (T, error) {
	service, err := c.Resolve(name)
	if err != nil {
		var zero T
		return zero, err
	}

	result, ok := service.(T)
	if !ok {
		var zero T
		return zero, fmt.Errorf("service '%s' cannot be cast to type %T", name, zero)
	}

	return result, nil
}

// MustResolve resolves a service and panics on error
func (c *DIC) MustResolve(name string) any {
	return safe.Must(c.Resolve(name))
}

// MustResolveTyped resolves a service with type safety and panics on error
func MustResolveTyped[T any](c *DIC, name string) T {
	return safe.Must(ResolveTyped[T](c, name))
}

// IsRegistered checks if a service is registered
func (c *DIC) IsRegistered(name string) bool {
	c.mutex.RLock()
	defer c.mutex.RUnlock()
	_, exists := c.services[name]
	return exists
}

// GetRegisteredServices returns all registered service names
func (c *DIC) GetRegisteredServices() []string {
	c.mutex.RLock()
	defer c.mutex.RUnlock()

	names := make([]string, 0, len(c.services))
	for name := range c.services {
		names = append(names, name)
	}
	return names
}

// Clear removes all registered services
func (c *DIC) Clear() {
	c.mutex.Lock()
	defer c.mutex.Unlock()

	c.services = make(map[string]*ServiceDescriptor)
	c.resolutionStack = make([]string, 0)
}

// Remove removes a specific service registration
func (c *DIC) Remove(name string) bool {
	c.mutex.Lock()
	defer c.mutex.Unlock()

	if _, exists := c.services[name]; exists {
		delete(c.services, name)
		return true
	}
	return false
}
