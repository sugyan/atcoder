open Base;;

let _ = Stdlib.read_int () in
let a =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let c = Array.create ~len:4 0 in
  List.iter a ~f:(fun x -> c.(x % 4) <- c.(x % 4) + 1);
  c.(1) + c.(3) + Bool.to_int (c.(2) > 0) <= c.(0) + 1
in
answer |> (function true -> "Yes" | false -> "No") |> Stdlib.print_endline
