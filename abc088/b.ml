open Base;;

let _ = Caml.read_int () in
let a =
  Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let f i acc x = acc + if i % 2 = 0 then x else -x in
  List.sort a ~compare:Int.descending |> List.foldi ~init:0 ~f
in
Int.to_string answer |> Stdio.print_endline
