(module
//Define the first function for addition as was done in previous example
(func (param $a i32) (param $b i32) (result i32)
   get_local $a 
   get_local $b 
   i32.add
)
//Define the 2nd function for subtraction of the two numbers. As explained above as well, the local variables are pushed to the stack and i32.sub then pops the value from the stack and subtracts the two and pushes the result to the stack again.
(func (param $a i32) (param $b i32) (result i32)
   get_local $a 
   get_local $b 
   i32.sub
)
// define the 3rd function for multiplication. Here i32.mul instruction pops the 2 values from the stack, multiplies them and pushes result back to the stack.
(func (param $a i32) (param $b i32) (result i32)
   get_local $a 
   get_local $b 
   i32.mul
)
  (export "add" (func 0))
  (export "subtract" (func 1))
  (export "multiply" (func 2))
)
