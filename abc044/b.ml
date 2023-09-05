open Base;;

let w = Stdlib.read_line () in
let answer =
  List.init 128 ~f:Char.of_int_exn
  |> List.for_all ~f:(fun c -> String.count w ~f:(Char.equal c) % 2 = 0)
in
answer |> (function true -> "Yes" | false -> "No") |> Stdlib.print_endline
