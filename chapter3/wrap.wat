(module
(func  (param $a i32) (param $b i32) (result i32)
   get_local $a
   get_local $b
   i32.add
)
(func (result i32)
   i32.const 56
   i32.const 44
   call 0
)

  (export "add1" (func 1))
)
