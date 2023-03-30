# Dialects

> **Warning**
> 
> Work in progress

The compiler contains different dialects. Every source file or module has a module type which represents the dialect. Every dialect should be binary-compatible with [Urban engine](https://github.com/sqyyy-jar/urban-engine/).

Every module contains the following:

* `file`: the name of the source file
* `src`: the source as a string
* `func_indices`: lookup table for function indices
	* May be replaced by dialect trait-functions
* `static_indices`: lookup table for static variable indices
	* May be replaced by dialect trait-functions
* `dialect`: the boxed instance of the dialect
	* Optional, is swapped with none during interaction

# To do

* Implement concept
* Write about tokenizer and parser