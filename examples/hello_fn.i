module main

let hello_world welcome y =
    let year = y + 20
    printfn "Hello " welcome year

let main () =
    hello_world "world" 2000
