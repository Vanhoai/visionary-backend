package safe

func Assert(condition bool, msg string) {
	if !condition {
		panic(msg)
	}
}
