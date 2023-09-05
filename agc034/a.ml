open Base;;

let a, b, c, d =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d %d %d %d" (fun _ a b c d ->
      (a, b, c, d))
in
let s = Stdlib.read_line () in
let answer =
  let f substring s = String.is_substring s ~substring in
  (c < d || String.sub s ~pos:(b - 2) ~len:(d - b + 3) |> f "...")
  && String.sub s ~pos:(a - 1) ~len:(c - a + 1) |> f "##" |> not
  && String.sub s ~pos:(b - 1) ~len:(d - b + 1) |> f "##" |> not
in
answer |> (function true -> "Yes" | false -> "No") |> Stdlib.print_endline
