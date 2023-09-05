open Base;;

let n = Stdlib.read_int () in
let answer =
  List.init n ~f:Int.succ
  |> List.fold ~init:1 ~f:(fun acc x -> acc * x % 1_000_000_007)
in
answer |> Int.to_string |> Stdlib.print_endline
