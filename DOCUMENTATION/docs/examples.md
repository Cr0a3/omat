# 
## if-statements
Omat
```
use std;

fn main() -> i32 {
    let ok: bool = true;

    if (ok == true) {
        std::out.println("you are not allowed");
    }  
    else {
        std::out.println("you are allowed");

    }

    return 0;
}
```

## for-loop
Omat
```
use std;

fn main() -> i32 {

    for (i: i32 = 0; i < 5; i++) {
        std::out.println("A");
    }

    return 0;
}
```

## foreach-loop
Omat
```
use std;

fn main() -> i32 {
    let vector: std::Vec<i32> = std::Vec<i32>::new();
    vector.push(5); // same as 'vector.push_back(5)'

    foreach (let elem: i32 in vector) { //iterates over all elements in vector
        std::out.println("{}", elem);
    }

}
```


<b>
When iterating over strings, each char get present
</b>

## while-loop
Omat
```
use std;

fn main() -> i32 {
    let running: bool = true;

    std::out.println("The machine");
    std::out.println("type in false to stop it");
    std::out.println("type in true to let it run on epoche");

    let epoch: i32 = 1;

    while (running) {
        std::out.println("it is running since {} epochs", epoch);
        epoch += 1;

        running = std::in.getBool();
    }

    return 0;
}
```

## functions
Omat
```
use std;

fn test() -> i32 { // function 'test' that returns an i32
    return 0;
}

fn test_with_arg(x: i32) -> i32 { //function 'test_with_arg' that has one i32 argument
    return x;
}

fn main() -> i32 {
    std::out.println("test() -> {}", test());
    std::out.println("test(5) -> {}", test_with_arg(5));

    return 0;
}

```

## enum
Omat
```
use std;

enum cars {
    ford,
    bmw,
    audi,
    merceds,
    unknow, // optional
};

fn main() -> i32 {
    let car: cars = cars::unknow;
    std::out.println("car = {}", car.type); //prints out car = unknow

    car = cars::merceds;
    std::out.println("new car = {}", car.type); //new car = mercedes
    return 0;
}
```

## switch
Omat
```
use std;

fn main() -> i32 {
    let users: i32 = 1_000_000; //same as 1000000

    switch users {
        > 100 => {  //more than 100 users
            std::out.println("more than 100 users");
        }

        99 => { //exactly 99 users
            std::out.println("exactly 99 users");
        }

        _ => {  //everything else
            std::out.println("i don't know what do write here");
            std::out.println("users = {}", users);
        }
    }
    return 0;
}
```

## structs
Omat
```
use std;

struct city {
    name: String,
    residents: i64,
}

fn main() -> i32 {
    let hamburg: city;
    hamburg.name = "Hamburg";
    hamburg.residents = 1_841_000;

    std::out.println("{} has {} residents", hamburg.name, hamburg.residents);

    return 0;
}

```

## classes
```
```

## templates
```
```

## references
```
```

## pointers
In omat you can create pointers just like in c.

Omat
```
use std;

fn set_var<t>(var: *<t>, value: <t>) {  // a pointer
    *var = value; // the value of the pointer
}

fn main() -> i32 {
    let someVariable: i32 = 100;
    std::out.print("someVariable = {}", someVariable);

    set_var<i32>(&someVariable, 101);   //gives an reference of the someVariable variable
    std::out.print("someVariable = {}", someVariable);
    
    return 0;
}
```

## error handling
```
```

## safty
In omat, you can create public and private functions, that are just aviable in the current module or class.
You also can create regions, with private variables.

Omat
```
module test;
pub fn test() -> i32 { /*...*/ }        //everywhere aviable
prv fn test_prv() -> i32 { /*...*/ }    //just aviable in this module/file
```

Omat
```
pub fn main() -> i32 {
    let x: i32 = 0;

    {
        let y: i32 = x + 1; // x is aviable here
    }

    x = y; // error because x does not exits in this scope

    return 0;
}
```


## including
other_file.om
```
use std;
module other_file;

pub fn func() { //can be used outside the module/file
    std::out.println("called public function other_file.om/func");
}

prv fn private_func() { //cannot be used outside the module/file
    std::out.println("called private function other_file.om/private_func");
}
```

main.om
```
import "other_file.om"; //direct importing
/*
or:
use other_file ;        //importing the other_file module
*/

fn main() -> i32 {
    other_file::func();
    other_file::private_func(); //error! because private_func does not exits outside the other_file module
    return 0;
}
```

## mixing with other languages

### c from omat
To mix C with omat, you can use the omat keyword <code>extern</code>.

Omat
```
use std::C::*;
use std;
extern {
    fn addC(Cint, Cint) -> Cint;
}
fn main() -> i32{
    let c: i32 = i32.from(addC(1, 1)); // calls c function + converts it to i32
    std::out.println("1 + 1 = {}", c);
}
```

C
```
int addC(int a, int b) {
    return a + b;
}
```

### omat from c

To call C from omat you need to put a <code>export</code> before your function.
Same example:

Omat
```
use std::C::*;
export fn addC(Cint a, Cint b) -> Cint {
    return a + b;
}
```

C
```
#include <stdio.h>

extern void addC(int, int);

int main() {
    int c = addC(1, 1);
    printf("1 + 1 = %d", c);
    return 0;    
}
```

To use omat from c you need to use the omatc-flags <code>--release -obj</code>
The <code>--release</code> flag is for maximum optimization and the <code>-obj</code>
flag is to set the file format to .o (object file).

## os developement
When you chose Omat for creating an os, you have the comfort of using the inbuild os libary.
The inbuild os libary provide many tools and functions.

Omat
```
use os;

export fn _entry() -> i32 { //entry function where to bootloader jumps in
    os::init(); //important else, no function is aviable

    let frameBuffer: os::frameBuf = os::graphics::getFrameBuf();
    frameBuffer.write("Hello World!");
    
    return 0; 
}
```

To use the os libary in your own os, you need to add following compiler flags:
    - <code>-static_os</code>   to staticly link in the os libary
    - <code>-bareMetal</code>   to set no os dependencys
    - <code>-obj</code>         to set the output format to .o
    - <code>-noMain</code>
    - <code>-startCode=_entry</code>

Then you need to say your bootloader to jump into that object file and exectute it. 