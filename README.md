# A Rusty Chain

A flexible, all n-gram (bi-grams+) Markov chain-based text generator. This used to be in python, and then I decided to learn Rust.

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