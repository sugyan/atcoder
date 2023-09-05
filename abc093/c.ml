open Base;;

let a, b, c =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d %d" (fun a b c -> (a, b, c))
in
let answer =
  let m = a |> max b |> max c in
  let n = m + (m % 2 lxor ((a + b + c) % 2)) in
  ((3 * n) - a - b - c) / 2
in
answer |> Int.to_string |> Stdlib.print_endline
