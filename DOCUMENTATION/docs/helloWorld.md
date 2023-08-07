# Hello World
Since the c programming language there is a tradition for programmers who starting with a new programming
language to write a "Hello World!" programm. 

When doing that with omat, we first need to create a new project (package). We do that with opack:
```
opack new HelloWorld
```
Then we need to navigate into that folder.
To build and run this package:
```
cd HelloWorld
opack run
```
That also creates an [unoptimized] excutable in the target/debug folder.

## Hello World code
To get it now to output Hello World, we need to open up src/main.om in your created project folder.
There you need to type in following code sample:
```
use std;

fn main(std::args args) -> i32 {
    std::out.println("Hello World!");
    return 0;
} 
```
