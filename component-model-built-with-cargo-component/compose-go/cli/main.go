package main

import (
	"fmt"

	"github.com/sammyne/wasm-examples/component-model/compose-go/cli/bindings"
)

func main() {
	sum := bindings.DocsCalculator0_1_0_CalculateEvalExpression("add")
	fmt.Println("sum =", sum)
}
