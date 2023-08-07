# 
## Compiler flags

### --input 
Specifies the input file to compile
```
omatc --input <path>
```

### --output (-o)
Specifies the output file where the application is saved
```
omatc --output <path>
```
```
omatc -o <path>
```

### --debug
Sets, that the application will be compiled in debug stage.
That means it will be unoptimized and has debug flags.
```
omatc --debug
```

### --release
Sets, that the application will be compiled in release stage.
That means it will be maximaly optimzed.
```
omatc --release
```

### --no_main
Sets, that it has no main function to compile.
Commonly used for os development and for libary.
```
omatc --no_main
```

### --bareMetal
When you chosse this option, no os depend stuff will be linked in.
It also disables the compled std module.
But you can use the os module now.
```
omatc --bareMetal
```

### --obj   (-c)
Sets, that the file format of the output file is an object file.
```
omatc --obj
```
```
omatc -c
```

### --static_os
Sets, that the os libary will be staticly linked in.
This option is commonly used for os development projects.
```
omatc --static_os
```

### --startCode
Sets, that the code that get executed when starting the application is out of following function.
It is commonly used for os development.
Example of directly jumping into the <code>_entry</code> function:
```
omatc --startCode _entry
```