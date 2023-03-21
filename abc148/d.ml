open Base;;

let n = Caml.read_int () in
let a =
  Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  List.fold a ~init:0 ~f:(fun acc x -> acc + if x = acc + 1 then 1 else 0)
  |> function
  | 0 -> -1
  | x -> n - x
in
answer |> Int.to_string |> Caml.print_endline
