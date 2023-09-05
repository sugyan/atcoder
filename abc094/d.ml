open Base;;

let _ = Stdlib.read_int () in
let a =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let n = List.fold a ~init:0 ~f:max in
  let f acc x = if abs (x - (n / 2)) < abs (acc - (n / 2)) then x else acc in
  (n, List.fold a ~init:0 ~f)
in
answer |> fun (n, r) -> Printf.sprintf "%d %d" n r |> Stdlib.print_endline
