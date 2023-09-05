open Base;;

let n = Stdlib.read_int () in
let a =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  List.fold a ~init:1 ~f:(fun acc x -> acc * (2 - (x % 2))) |> ( - ) (3 ** n)
in
answer |> Int.to_string |> Stdlib.print_endline
