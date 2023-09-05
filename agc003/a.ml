open Base;;

let s = Stdlib.read_line () in
let answer =
  let f = String.contains s in
  Bool.(f 'N' = f 'S' && f 'E' = f 'W')
in
answer |> (function true -> "Yes" | false -> "No") |> Stdlib.print_endline
