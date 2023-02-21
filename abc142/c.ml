open Base;;

let n = Caml.read_int () in
let a =
  Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let b = Array.create ~len:n "" in
  List.iteri a ~f:(fun i x -> b.(x - 1) <- i + 1 |> Int.to_string);
  b |> Array.to_list
in
answer |> String.concat ~sep:" " |> Stdio.print_endline
