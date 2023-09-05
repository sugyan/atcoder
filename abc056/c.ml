open Base;;

let x = Stdlib.read_int () in
let answer =
  let rec f i n = if n >= x then i else f (i + 1) (n + i + 1) in
  f 1 1
in
answer |> Int.to_string |> Stdlib.print_endline
