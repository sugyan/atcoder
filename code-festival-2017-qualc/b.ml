open Base;;

let n = Caml.read_int () in
let a =
  Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  List.fold a ~init:1 ~f:(fun acc x -> acc * (2 - (x % 2))) |> ( - ) (3 ** n)
in
answer |> Int.to_string |> Caml.print_endline
