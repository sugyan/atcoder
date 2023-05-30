open Base;;

let n = Caml.read_int () in
let a =
  Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let a = (0 :: a) @ [ 0 ] |> List.to_array in
  let d = Array.init (n + 1) ~f:(fun i -> abs (a.(i) - a.(i + 1))) in
  let sum = Array.fold d ~init:0 ~f:( + ) in
  List.init n ~f:(fun i -> sum + abs (a.(i) - a.(i + 2)) - d.(i) - d.(i + 1))
in
answer |> List.iter ~f:(Fn.compose Caml.print_endline Int.to_string)
