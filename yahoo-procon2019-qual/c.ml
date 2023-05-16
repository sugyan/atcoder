open Base;;

let k, a, b =
  Caml.Scanf.sscanf (Caml.read_line ()) "%d %d %d" (fun k a b -> (k, a, b))
in
let answer =
  if a + 2 < b && k > a then a + ((k - a + 1) / 2 * (b - a)) + ((k - a + 1) % 2)
  else k + 1
in
answer |> Int.to_string |> Caml.print_endline
