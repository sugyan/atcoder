open Base;;

let _ = Stdlib.read_int () in
let a =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let f (sum, c) x = (sum + x, if sum * 2 < x then 1 else c + 1) in
  List.sort a ~compare |> List.fold ~init:(0, 0) ~f |> snd
in
answer |> Int.to_string |> Stdlib.print_endline
