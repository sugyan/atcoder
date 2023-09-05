open Base;;

let a = Stdlib.read_line () in
let b = Stdlib.read_line () in
let answer =
  (compare (String.length a) (String.length b) |> function
   | 0 -> Poly.compare a b
   | o -> o)
  |> function
  | 1 -> "GREATER"
  | 0 -> "EQUAL"
  | _ -> "LESS"
in
answer |> Stdlib.print_endline
