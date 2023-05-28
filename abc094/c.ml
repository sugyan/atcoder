open Base;;

let n = Caml.read_int () in
let x =
  Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let a = List.sort x ~compare |> List.to_array in
  List.map x ~f:(fun x -> if a.(n / 2) > x then a.(n / 2) else a.((n / 2) - 1))
in
answer |> List.iter ~f:(Fn.compose Caml.print_endline Int.to_string)
