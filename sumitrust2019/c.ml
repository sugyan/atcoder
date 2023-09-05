open Base;;

let x = Stdlib.read_int () in
let answer = x % 100 <= x / 100 * 5 in
answer |> (function true -> "1" | false -> "0") |> Stdlib.print_endline
