open Base;;

let x = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun _ x -> x) in
let xs =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let rec gcd a b = if b = 0 then a else gcd b (a % b) in
  List.map xs ~f:(( - ) x) |> List.map ~f:abs |> List.fold ~init:0 ~f:gcd
in
answer |> Int.to_string |> Stdlib.print_endline
