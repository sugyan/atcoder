open Base;;

let n, a, b =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d %d" (fun n a b -> (n, a, b))
in
let answer =
  if (b - a) % 2 = 0 then (b - a) / 2
  else min (a - 1) (n - b) + 1 + ((b - a - 1) / 2)
in
answer |> Int.to_string |> Stdlib.print_endline
