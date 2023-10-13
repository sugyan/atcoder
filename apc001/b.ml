open Base;;

let _ = Stdlib.read_int () in
let f _ =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let a = f () in
let b = f () in
let answer =
  let f (a, b) = (a - b) / if a > b then 1 else 2 in
  List.zip_exn a b |> List.sum (module Int) ~f |> ( >= ) 0
in
answer |> (function true -> "Yes" | false -> "No") |> Stdlib.print_endline
