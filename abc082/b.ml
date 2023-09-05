open Base;;

let s = Stdlib.read_line () in
let t = Stdlib.read_line () in
let answer =
  let f ~compare = Fn.compose (List.sort ~compare) String.to_list in
  Poly.(f s ~compare:ascending < f t ~compare:descending)
in
answer |> (function true -> "Yes" | false -> "No") |> Stdlib.print_endline
