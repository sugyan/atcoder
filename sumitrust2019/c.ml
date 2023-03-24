open Base;;

let x = Caml.read_int () in
let answer = x % 100 <= x / 100 * 5 in
answer |> (function true -> "1" | false -> "0") |> Caml.print_endline
