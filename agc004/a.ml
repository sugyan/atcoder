open Base;;

let a, b, c =
  Caml.Scanf.sscanf (Caml.read_line ()) "%d %d %d" (fun a b c -> (a, b, c))
in
let answer = a % 2 * b % 2 * c % 2 * (a * b |> min (b * c) |> min (c * a)) in
answer |> Int.to_string |> Caml.print_endline
