(module
  (import "example" "add" (func $add (param i32) (param i32)))
  (func (export "add1")
    i32.const 56
    i32.const 44

    call $add))
