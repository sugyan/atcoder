open Base;;

let x = Caml.read_int () in
let answer =
  (x / 11 * 2) + match x % 11 with 0 -> 0 | x when x < 7 -> 1 | _ -> 2
in
answer |> Int.to_string |> Caml.print_endline
