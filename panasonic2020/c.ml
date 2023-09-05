open Base;;

let a, b, c =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d %d" (fun a b c -> (a, b, c))
in
let answer =
  let d = c - a - b in
  d > 0 && a * b * 4 < d * d
in
answer |> (function true -> "Yes" | false -> "No") |> Stdlib.print_endline
