open Base;;

let _ = Stdlib.read_int () in
let a =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let b =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let a = List.to_array a in
  List.foldi b ~init:0 ~f:(fun i acc x ->
      let y = min a.(i) x in
      let z = min a.(i + 1) (x - y) in
      a.(i + 1) <- a.(i + 1) - z;
      acc + y + z)
in
answer |> Int.to_string |> Stdlib.print_endline
