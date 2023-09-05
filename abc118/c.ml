open Base;;

let _ = Stdlib.read_int () in
let a =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let rec f x y = if y = 0 then x else f y (x % y) in
  List.fold a ~init:0 ~f
in
answer |> Int.to_string |> Stdlib.print_endline
