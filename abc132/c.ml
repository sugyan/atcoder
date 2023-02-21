open Base;;

let n = Caml.read_int () in
let d =
  Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let d = List.sort d ~compare |> List.to_array in
  d.(n / 2) - d.((n / 2) - 1)
in
answer |> Int.to_string |> Stdio.print_endline
