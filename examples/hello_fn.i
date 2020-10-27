module main

let x () = printfn "Test func"

let y (val1) = 
    let v = val1 * 20
    x()
    v

let z = 1 + y()

let hello_world welcome y =
    let year = y + 20
    printfn "Hello " welcome year

let main () =
    hello_world "world" 2000
