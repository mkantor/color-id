# color-id

A CLI application that reads colors from standard input and finds the 
perceptually-nearest color out of a set of "parent" colors specified in a 
configuration file.

## Quick start

This repository contains a handy script to build the application from source.
Here's how to use it to test a single color. First make sure you have git and 
docker installed, then:

```sh
git clone https://github.com/mkantor/color-id.git
cd color-id
echo "ff0000" | ./util/build-and-run
```

Expect this to be painfully slow the first time as it has to download the Rust 
compiler and all dependencies, but should be pretty quick for subsequent runs.

## Utilities

There are a few other scripts included for playing with the application. Here's
a few things you can do with them:

### Visualize Results

Generate an HTML page visualizing 1000 input/output pairs:

```sh
./util/sample 1000 | ./util/visualize > output.html
```
