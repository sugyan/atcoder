open Base;;

let n = Stdlib.read_int () in
let answer =
  let rec f n l0 l1 = if n = 0 then l0 else f (n - 1) l1 (l0 + l1) in
  f n 2 1
in
answer |> Int.to_string |> Stdlib.print_endline
