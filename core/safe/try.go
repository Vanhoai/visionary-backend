package safe

import "fmt"

func Try[T any](fn func() (T, error)) (result T, err error) {
	defer func() {
		if r := recover(); r != nil {
			if e, ok := r.(error); ok {
				err = e
			} else {
				err = fmt.Errorf("panic: %v", r)
			}
		}
	}()

	return fn()
}
