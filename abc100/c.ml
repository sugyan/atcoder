open Base;;

let _ = Caml.read_int () in
let a =
  Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let rec f x = if x % 2 = 0 then 1 + f (x / 2) else 0 in
  List.sum (module Int) a ~f
in
answer |> Int.to_string |> Caml.print_endline
