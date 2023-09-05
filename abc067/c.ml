open Base;;

let _ = Stdlib.read_int () in
let a =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let sum = List.fold a ~init:0 ~f:( + ) in
  List.folding_map a ~init:0 ~f:(fun acc x -> (acc + x, (acc * 2) - sum |> abs))
  |> List.tl_exn
  |> List.fold ~init:Int.max_value ~f:min
in
answer |> Int.to_string |> Stdlib.print_endline
