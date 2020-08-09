```
main = (
        namespace |
        module    |
        function  |
        let-binding 
    )*

// Basic rules
ident = alpha+ (alphanum | "_")* 

// Namespaces declaration
namespace = "namespace" ["rec"] (parent-namespaces ".")* namespace-name
parent-namespaces = ident
namespace-name = ident

// Module declaration
module = "module" [accessibility-modifier] (qualified-namespace "." )* module-name
accessibility-modifier = ("public" | "internal" | "private")
qualified-namespace = indent
module-name = ident

// Function declarations
function = "let" [ ("inline" | "rec") ] function-name parameter-list [ ":" return-type ] "=" function-body
function-name = ident
parameter-list = (parameter-value-list+ | parameter-list-brackets)
parameter-list-brackets = "(" [(
            parameter-value |
            parameter-value-type
        ) [","]]* ")"
parameter-value-list = (parameter-value | parameter-list-brackets)
parameter-value-type = (parameter-value ":" parameter-type | "(" parameter-value ":" parameter-type ")")
parameter-value = (ident | "(" ident ")")
parameter-type = ((ident | "(" ident ")") ["*"])+
return-type = parameter-type 
function-body = [(let-binding | function-call)*] return-statement
return-statement = func-value

// Let binding
let-binding = "let" let-value-list "=" function-body
let-value-list = (parameter-value-list [","])+
value-list = (parameter-value | "(" (parameter-value [","])* ")")

// Function call statements
function-call = function-call-name (func-value+ | "(" func-value-comma* ")")
function-call-name = (function-name ".")* function-name
func-value = (value-list | "(" expression ")")
func-value-comma = func-value [","]

// Expression declarations
expression = (
            func-value | 
            function-call | 
            "(" function-call ")"
        ) [expression-operations expression]
expression-operations = (
            "+" | "-" |
            "*" | "/" |
            "<<<" | ">>>"
        ) 
```