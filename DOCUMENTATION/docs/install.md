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
