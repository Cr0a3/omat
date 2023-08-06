# The toolchain
At the current build there are 3 programms in the toolchain: <br>

 - oexp <br>
 - opack <br>
 - omatc <br>

## oexp
The tool oexp provides error explenations for all errors that occure in a omat toolchain programm.
you can custumize the oexp programm for your own programming language by edditing the oexp_exps.json file in the data folder, in the installation.

Example using:
```
oexp explain Ee000
```

explans the error 000 in the oexp programm


## opack
With opack you can build und manage your project. 
It provides commands to build and create an package.
It uses to build the omat compiler (omatc).

Example using:
```
opack new test
```

creates an project (or package) with the name test.

## omatc
The omat compiler, it compiles the omat code into an binary application.
The opack programm uses it to build tge current package.
