package main

import (
	"context"

	"io/ioutil"
	"os"
	"log"
	"net/http"

	"github.com/wapc/wapc-go"


	json2msgpack "github.com/izinin/json2msgpack"
)

var ctx context.Context
func main() {
	wasmname := os.Args[1]
        functionname := os.Args[2]
	code, err := ioutil.ReadFile(wasmname)
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

	http.HandleFunc("/", testHandler(instance,functionname))
        log.Fatal(http.ListenAndServe(":8080", nil))





}

func testHandler(instance *wapc.Instance, functionname string ) http.HandlerFunc {
return func (w http.ResponseWriter, r *http.Request) {

   keys, ok := r.URL.Query()["key"]
  ctx := context.Background()  
    if !ok || len(keys[0]) < 1 {
        log.Println("Url Param 'key' is missing")
        return
    }

    // Query()["key"] will return an array of items, 
    // we only want the single item.
    key := keys[0]

b:=json2msgpack.EncodeJSON([]byte(key))

//        fmt.Println(b)


        result, err := instance.Invoke(ctx, functionname, b)
        if err != nil {
                panic(err)
        }
w.Write([]byte(result))
//        fmt.Println(w, string(result))
}
}



