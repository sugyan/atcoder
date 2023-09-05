open Base;;

let n = Stdlib.read_int () in
let a =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let a = List.to_array a in
  List.range 0 n
  |> List.count ~f:(fun i -> a.(a.(i) - 1) - 1 = i)
  |> Fn.flip ( / ) 2
in
answer |> Int.to_string |> Stdlib.print_endline
