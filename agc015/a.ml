open Base;;

let n, a, b =
  Caml.Scanf.sscanf (Caml.read_line ()) "%d %d %d" (fun n a b -> (n, a, b))
in
let answer =
  if a > b || (n = 1 && a <> b) then 0 else ((n - 2) * (b - a)) + 1
in
answer |> Int.to_string |> Caml.print_endline
