# Hello World
Since the c programming language it is a tradition for programmers who starting with a new programming
language to write a "Hello World!" programm. 

When doing that with omat, whe first need to create an new project (package). We do that with opack:
<code>
opack new HelloWorld
</code>
Then we need to navigate into that folder.
To build and run this package you need to run:
<code>
opack run
</code>
That also creates an [unoptimized] excutable in the target/debug folder.

## Hello World code
To get it now to output Hello World, we need to open up src/main.om in your created project folder.
There you need to type in following code sample:
<code>
use std;

fn main(std::args args) -> int32 {
    std::out.println("Hello World!");
    return 0;
} 
</code>
