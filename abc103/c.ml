open Base;;

let n = Caml.read_int () in
let a =
  Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer = List.sum (module Int) a ~f:Fn.id - n in
answer |> Int.to_string |> Caml.print_endline
