# iLang Grammar

Formal grammar based on EBNF.

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
namespace = "namespace" ["rec"] (namespace-name ".")* namespace-name
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
parameter-value = ident-value 
ident-value = (ident | "(" ident ")") 
parameter-type = (ident-value ["*" ident-value] | "(" ident-value ["*" ident-value] ")")+
return-type = parameter-type 
function-body = [function-body-statement]* return-statement
function-body-statement = (let-binding | function-call)
return-statement = function-value

// Let binding
let-binding = "let" let-value-list "=" function-body
let-value-list = (parameter-value-list [","])+
value-list = (parameter-value | "(" (parameter-value [","])* ")")

// Function call statements
function-call = function-call-name (function-value+ | "(" [function-value [","] ]* ")")
function-call-name = (function-name ".")* function-name
function-value = (value-list | "(" expression ")")

// Expression declarations
expression = (
            function-value | 
            function-call | 
            "(" function-call ")"
        ) [expression-operations expression]
expression-operations = (
            "+" | "-" |
            "*" | "/" |
            "<<<" | ">>>"
        ) 
```
