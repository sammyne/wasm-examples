package main

import (
	"fmt"
	"reflect"

	"github.com/sammyne/wasm-examples/component-model/hello-world/component/bindings"
)

func init() {
	w := ExampleWorld{}
	bindings.SetBindings(w)
}

func main() {}

type ExampleWorld struct{}

func (w ExampleWorld) HelloWorld() string {
	t := reflect.TypeFor[ExampleWorld]()

	out := fmt.Sprintf("%s-%s", t.Name(), t.PkgPath())

	return "hello world :)" + out
}
