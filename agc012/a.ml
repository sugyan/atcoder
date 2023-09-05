open Base;;

let n = Stdlib.read_int () in
let a =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let a = List.sort a ~compare:descending |> List.to_array in
  List.init n ~f:(fun i -> a.((i * 2) + 1)) |> List.sum (module Int) ~f:Fn.id
in
answer |> Int.to_string |> Stdlib.print_endline
