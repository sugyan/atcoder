open Base;;

let n = Caml.read_int () in
let a0 =
  Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let a1 =
  Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  List.range 0 n
  |> List.map ~f:(fun i ->
         List.take a0 (i + 1) @ List.drop a1 i |> List.sum (module Int) ~f:Fn.id)
  |> List.fold ~init:0 ~f:max
in
answer |> Int.to_string |> Caml.print_endline
