open Base;;

let n = Caml.read_int () in
let a =
  Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let a = List.to_array a in
  List.range 0 n
  |> List.count ~f:(fun i -> a.(a.(i) - 1) - 1 = i)
  |> Fn.flip ( / ) 2
in
answer |> Int.to_string |> Caml.print_endline
