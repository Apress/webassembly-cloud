package main

import (
	"context"
	"fmt"
	"io/ioutil"
	"os"
	"github.com/wapc/wapc-go"
	json2msgpack "github.com/izinin/json2msgpack"
)
//var  instance  wasm.Instance
var ctx context.Context
func main() {
	if len(os.Args) < 2 {
		fmt.Println("usage: hello <name>")
		return
	}
        wasm:=os.Args[1]
	funcname := os.Args[2]
	name:=os.Args[3]
	ctx := context.Background()
	code, err := ioutil.ReadFile(wasm)
	if err != nil {
		panic(err)
	}

	module, err := wapc.New(code,nil)
	if err != nil {
		panic(err)
	}
	
	defer module.Close()

	instance, err := module.Instantiate()
	if err != nil {
		panic(err)
	}
	defer instance.Close()
	b:=json2msgpack.EncodeJSON([]byte(name))

	result, err := instance.Invoke(ctx, funcname, b)
	if err != nil {
		panic(err)
	}

	fmt.Println(string(result))
}
