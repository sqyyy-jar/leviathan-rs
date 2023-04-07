# Dialects

> **Warning**  
> Work in progress

The compiler contains different dialects. Every source file or module has a module type which represents the dialect. Every dialect should be binary-compatible with [Urban engine](https://github.com/sqyyy-jar/urban-engine/).

Every module contains the following:

* `file`: the name of the source file
* `src`: the source as a string
* `dialect`: the boxed instance of the dialect
	* Optional, is swapped with none during interaction
* `lookup_callable`: function to lookup callable elements of a dialect
	* returns an index to the element
	* takes name of the element as parameter
	* only returns if element is public
* (in the future) `lookup_static`
* (WIP) `callable_signature`: function to get signature of callable element

# To do

* Write about tokenizer and parser