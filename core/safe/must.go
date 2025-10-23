package safe

func Must[T any](value T, err error) T {
	if err != nil {
		panic(err)
	}

	return value
}

func MustNoValue(err error) {
	if err != nil {
		panic(err)
	}
}
