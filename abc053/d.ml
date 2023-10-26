open Base;;

let _ = Stdlib.read_int () in
let a =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  List.dedup_and_sort a ~compare |> List.length |> fun x -> x - 1 + (x % 2)
in
answer |> Int.to_string |> Stdlib.print_endline
