open Base;;

let _ = Stdlib.read_int () in
let k = Stdlib.read_int () in
let xs =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let f x = Int.min x (k - x) |> ( * ) 2 in
List.sum (module Int) xs ~f |> Int.to_string |> Stdio.print_endline
