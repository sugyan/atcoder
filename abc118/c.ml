open Base;;

let _ = Caml.read_int () in
let a =
  Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let rec f x y = if y = 0 then x else f y (x % y) in
  List.fold a ~init:0 ~f
in
answer |> Int.to_string |> Caml.print_endline
