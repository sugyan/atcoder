open Base;;

let n = Caml.read_int () in
let answer =
  List.range 1 (n + 1)
  |> List.fold ~init:1 ~f:(fun acc x -> acc * x % 1_000_000_007)
in
answer |> Int.to_string |> Caml.print_endline
