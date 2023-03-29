open Base;;

let q, h, s, d =
  Caml.Scanf.sscanf (Caml.read_line ()) "%d %d %d %d" (fun q h s d ->
      (q, h, s, d))
in
let n = Caml.read_int () in
let answer =
  let s = s |> min (q * 4) |> min (h * 2) in
  let d = d |> min (s * 2) in
  (n / 2 * d) + (n % 2 * s)
in
answer |> Int.to_string |> Caml.print_endline
