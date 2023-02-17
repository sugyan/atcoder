open Base;;

let _ = Caml.read_int () in
let k = Caml.read_int () in
let xs =
  Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let f x = Int.min x (k - x) |> ( * ) 2 in
List.sum (module Int) xs ~f |> Int.to_string |> Stdio.print_endline
