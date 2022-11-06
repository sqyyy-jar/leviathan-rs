# Leviathan
Leviathan is a Lisp-style programming language written in Rust.
It currently is in early development and will just be a fun project of me.

## Features
* Abstract Syntax Tree Parsing
  * Nodes
  * Lists
  * Identifiers
  * Atoms
  * Strings
  * Integers
  * Floats
  * Comments

### Node
A node looks like this:
```clj
(operator arguments)
```
The operator must be a valid identifier and each argument can be any element named above, seperated by *whitespace.

### List
A list looks like this:
```clj
[:1 2 "3"]
```

### Identifier
An identifier is just text that can not be parsed in any other way:
```clj
# the operator is always an identifier
# the first argument is an integer
# the second argument is a string
# the last argument is an identifier
(operator 1 "2" three)
```

### Atom
An atom is a type that holds itself.
It is written as a colon ':' followed by an [identifier](#identifier):
```clj
:atom
```
The difference is that identifiers exist to represent e.g. variables and atoms are standalone.
Therefore atoms can be empty (`:`).

### String
A string is UTF-8 encoded text surrounded by double quotes '"'.
There are the following escape-characters:
* '\n' => newline
* '\r' => carriage return
* '\t' => tab
* '\\"' => double quote
* '\\\\' => backslash

An example:
```clj
"some text"
"\n"
"\r"
"\t"
"\""
"\\"
```
Newlines inside the string are not supported.

### Number
A number might be an integer or a floating point number:
```clj
# the first two arguments are floats and the second two are integers
-1.
-.5
2.
.5
-3
4
```

### Comment
A comment is a hashtag '#' followed by the comment till the end of the line:
```clj
# this is a comment
```

## Footnotes
* *whitespace: whitespaces include the comma character ','
