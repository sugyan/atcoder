open Base;;

let _ = Stdlib.read_int () in
let a =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  List.map a ~f:Int.ctz |> List.min_elt ~compare |> Option.value_exn
in
answer |> Int.to_string |> Stdlib.print_endline
