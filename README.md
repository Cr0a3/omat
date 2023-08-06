# omat
The omat language

# What is omat?
Omat is a simple hobby programming language with a small toolchain and
a build system.

## The toolchain
At the current build there are 3 programms in the toolchain: <br>

 - oexp <br>
 - opack <br>
 - omatc <br>

## Features
- Static type system
- Safe and fast runtime
- Perfect for os development
- Easy to mix with other languages
- Easy to learn for Rust programmers
- LLVM backend

# Installation guide
To install the omat toolchain, you have two ways of doing that.
The first one is, to download the prebuilded toolchain for windows, extract it, 
save it to your disk and then add the bin directory to your system enviroment variables. 

<b> [link to prebuilded windows binarys]  </b>

When you do not have a windows computer you build the toolchain for yourself

## Build the omat toolchain
First, you need to download the source code.

<b> [link to source code] </b>

The omat toolchain is written in rust, so you need to install rust:
<code>
sudo apt install cargo -y
</code>
than you can go into each subdirectory (not the docs directory) and run:
<code>
cargo build --release
</code>
Than in each subdirectory (not the docs directory) is a folder with the name target, in there is a folder with the name release
go into that folders and copy the excutable into the bin directory.
After the build you need to add the bin directory to the system enviroment variables.
