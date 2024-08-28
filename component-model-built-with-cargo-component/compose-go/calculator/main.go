package main

import "github.com/sammyne/wasm-examples/component-model/compose-go/calculator/bindings"

func init() {
	var w World
	bindings.SetExportsDocsCalculator0_1_0_Calculate(w)
}

func main() {}

type World struct{}

func (w World) EvalExpression(_ string) uint32 {
	return bindings.DocsAdder0_1_0_AddAdd(123, 456)
}
