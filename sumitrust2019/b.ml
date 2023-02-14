open Core;;

let solve n =
  let f i = i * 108 / 100 = n in
  List.range 1 50000 |> List.find ~f
  |> (function Some i -> Int.to_string i | None -> ":(")
  |> print_endline
in
Scanf.scanf "%d" solve
