open Base;;

let n = Stdlib.read_int () in
let a =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer = List.sum (module Int) a ~f:Fn.id - n in
answer |> Int.to_string |> Stdlib.print_endline
