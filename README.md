# color-id

A CLI application that reads colors from standard input and finds the 
perceptually-nearest color out of a set of "parent" colors specified in a 
configuration file.

This is my entry for @jonrahoi's [niftyColors contest](https://github.com/jonrahoi/niftyColors).

## Quick Start

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

There are some other scripts included for playing with the application. Here's
a few things you can do with them:

### Visualize Results

Generate an HTML page visualizing 1000 input/output pairs:

```sh
./util/sample 1000 | ./util/visualize > output.html
```

### Benchmarking

See how fast the program is for some number of inputs. There is some fixed 
startup cost (e.g. parsing the config file) so larger numbers of input colors 
should result in better performance-per-color. Note that you need the Rust 
compiler toolchain on your host machine to run benchmark.

```sh
./util/benchmark 1000000
```

My laptop churns through a million colors in about 11 seconds (so around 11 
microseconds per color). I don't mean to brag here; this is mostly thanks to 
Rust being "fast by default". I haven't gone out of my way to optimize anything 
yet (pull requests welcome!).
