open Base;;

let _ = Stdlib.read_int () in
let a =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let b = Array.create ~len:100_002 0 in
  List.iter a ~f:(fun x ->
      List.iter [ x; x + 1; x + 2 ] ~f:(fun i -> b.(i) <- b.(i) + 1));
  Array.fold b ~init:0 ~f:max
in
answer |> Int.to_string |> Stdlib.print_endline
