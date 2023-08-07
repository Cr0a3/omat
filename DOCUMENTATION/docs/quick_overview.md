# A quick overview over the inbuild libarys

## std
The std libary provides standard functions and is based on the c libary.

## c
The C libary provides every function out of the C Standard libary.
With the C types.

To include it type following code:
```
use c::*;
```
That includes everything out of the c libary.

### example
Here is an example that adds two Cints together and print that out via printf.
```
use c::*;

fn main() -> i32 {
    let a: Cint = Cint::from(5);
    let b: Cint = Cint::from(5);
    
    let c: Cint = a + b;

    stdio::printf("%d + %d = %d", a, b, c);
    return 0;
}
```

## os
The os libary provides functions to build your own operating system kernel and low level functions.