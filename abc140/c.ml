open Base;;

let n = Caml.read_int () in
let b =
  Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let b = List.to_array b in
  (List.range 0 (n - 2)
  |> List.sum (module Int) ~f:(fun i -> min b.(i) b.(i + 1)))
  + b.(0)
  + b.(n - 2)
in
answer |> Int.to_string |> Caml.print_endline
