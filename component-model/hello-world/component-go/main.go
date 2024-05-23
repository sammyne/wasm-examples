package main

import "github.com/sammyne/wasm-examples/component-model/hello-world/component/bindings"

func init() {
	w := ExampleWorld{}
	bindings.SetBindings(w)
}

func main() {}

type ExampleWorld struct{}

func (w ExampleWorld) HelloWorld() string {
	return "hello world :)"
}
