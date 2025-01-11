# LIST

My first project ever written in rust.

Rewrite of the ```ls``` command that is usually packaged with every operating system.

Written on Windows so file paths might not work as intended on other operating systems.

## How to compile:
Run ```cargo build``` with the release flag to get the binary,
```
$ cargo build --release
```
or you can just ```cargo run``` it
```
$ cargo run -- *args*
```

## How to run:
You can run it without arguments: 
```
$ ./list
```
```
$ cargo run
```
Prints out all of the files in the current directory, directory names are blue.
```
$ ./list *PATH*
```
```
$ cargo run -- *PATH*
```
Prints out all of the files in the path, panics if the path doesn't exist or if more than 1 argument is given.