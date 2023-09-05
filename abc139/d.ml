open Base;;

let n = Stdlib.read_int () in
let answer = n * (n - 1) / 2 in
answer |> Int.to_string |> Stdlib.print_endline
