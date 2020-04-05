# A Rusty Chain

[![Build Status](https://travis-ci.com/dluman/markov.svg?branch=master)](https://travis-ci.com/dluman/markov)

A flexible, all n-gram Markov chain-based text generator. This used to be in Python, and then I decided to learn Rust. Hence, this code isn't "idiomatic" in the way that it may eventually be. However, it works for my personal purposes as of now.

## Notes on usage

This (currently) requires three parameters:

* Text file to read
* Order to use to create chain
    * Integer
* Number of characters to generate
    * Integer
    
Once compiled, its standard usage is:

```bash
./markov {TEXT} {ORDER} {CHARACTERS TO GENERATE}
```

## Watch This Space

See above.