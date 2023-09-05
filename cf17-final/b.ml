open Base;;

let s = Stdlib.read_line () in
let answer =
  let f c = String.count s ~f:(Char.( = ) c) in
  let a, b, c = (f 'a', f 'b', f 'c') in
  (a |> max b |> max c) - (a |> min b |> min c) < 2
in
answer |> (function true -> "YES" | false -> "NO") |> Stdlib.print_endline
