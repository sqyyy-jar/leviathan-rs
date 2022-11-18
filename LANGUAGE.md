# The language
Note that this document - for now - is only for development purposes.

## Function structure
`(fn <NAME> <PARAMS> <RETURN-TYPE?> <TAGS?> <BODY>)`  
__minimal:__ `(fn <NAME> <PARAMS> <BODY>)`

__NAME__ is an identifier representing the name of the function.  
__PARAMS__ is a list representing the arguments of the functions.
__RETURN-TYPE__ is an identifier representing the return type of the function.  
__TAGS__ is a list representing additional metainformation of the function.  
__BODY__ is any node representing the body of the function.  
If the return-type is not provided, it will be unit (also known as void).  