open Base;;

let n = Caml.read_int () in
let answer = n * (n - 1) / 2 in
answer |> Int.to_string |> Caml.print_endline
