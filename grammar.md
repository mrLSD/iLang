```
namespace = "namespace" [rec] (parent-namespaces ".")* namespace-name
parent-namespaces = ident
namespace-name = ident

module = "module" [accessibility-modifier] (qualified-namespace "." )* module-name
accessibility-modifier = ("public" | "internal" | "private")
qualified-namespace = indent
module-name = ident

function = "let" [ ("inline" | "rec") ] function-name parameter-list [ ":" return-type ] "=" function-body
function-name = ident
parameter-list = (parameter-list-pure | parameter-list-brackets)
parameter-list-brackets = "(" [(parameter-value-type-brackets | parameter-value-type-without-brackets) [","]]* ")"
parameter-list-pure = (parameter-value-type-brackets)+
parameter-value-type-brackets = (parameter-value | "(" (parameter-value | parameter-value-type) ")" )
parameter-value-type-without-brackets = (parameter-value | parameter-value-type)
parameter-value-type = parameter-value ":" parameter-type
parameter-value = ident
parameter-type = ident

function-body = [let-binding]* return-statement
let-binding = "let" let-value-list "=" function-body
return-statement =
let-value-list = (let-value [","]  
let-value = ident