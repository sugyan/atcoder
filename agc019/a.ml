open Base;;

let q, h, s, d =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d %d %d" (fun q h s d ->
      (q, h, s, d))
in
let n = Stdlib.read_int () in
let answer =
  let s = s |> min (q * 4) |> min (h * 2) in
  let d = d |> min (s * 2) in
  (n / 2 * d) + (n % 2 * s)
in
answer |> Int.to_string |> Stdlib.print_endline
