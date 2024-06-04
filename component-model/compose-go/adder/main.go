package main

import "github.com/sammyne/wasm-examples/component-model/compose-go/adder/bindings"

func init() {
	var w World
	bindings.SetExportsDocsAdder0_1_0_Add(w)
}

func main() {}

type World struct{}

func (w World) Add(a uint32, b uint32) uint32 {
	return a + b
}
