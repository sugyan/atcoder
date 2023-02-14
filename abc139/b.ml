open Core;;

let f a b = (a + b - 3) / (a - 1) in
Scanf.scanf "%d %d" f |> Printf.sprintf "%d" |> print_endline
