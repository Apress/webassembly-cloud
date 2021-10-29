(module
  (table 3 funcref)
(func $f1 (param $a i32) (param $b i32) (result i32)
   get_local $a
   get_local $b
   i32.add
)
(func $f2 (param $a i32) (param $b i32) (result i32)
   get_local $a
   get_local $b
   i32.sub
)
(func $f3 (param $a i32) (param $b i32) (result i32)
   get_local $a
   get_local $b
   i32.mul
)

  (elem (i32.const 0) $f1 $f2 $f3) // this refers to the index of the function in the function table
  (type $return_i32 (func (param i32 i32)(result i32)))
  (func (export "callByIndex") (param  i32 i32 i32)(result i32)
local.get 1
local.get 2
local.get 0
    call_indirect (type $return_i32)
)
)
