open Base;;

let s = Caml.read_line () in
let t = Caml.read_line () in
let answer =
  let f ~compare = Fn.compose (List.sort ~compare) String.to_list in
  Poly.(f s ~compare:ascending < f t ~compare:descending)
in
answer |> (function true -> "Yes" | false -> "No") |> Caml.print_endline
