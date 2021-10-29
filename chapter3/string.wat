(module
  (import "example" "log" (func $log (param i32 i32)))
  (import "js" "mem" (memory 1))
  (data (i32.const 0) "Hello Wat")
  (func (export "logme")
    i32.const 0  ;; pass offset 0 to log
    i32.const 9  ;; pass length 9 to log
    call $log))
