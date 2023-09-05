open Base;;

let n = Stdlib.read_int () in
let s = Stdlib.read_line () in
let t = Stdlib.read_line () in
let answer =
  List.range 0 (n + 1)
  |> List.rev
  |> List.find_exn ~f:(fun i ->
         String.(sub s ~pos:(n - i) ~len:i = sub t ~pos:0 ~len:i))
  |> ( - ) (2 * n)
in
answer |> Int.to_string |> Stdlib.print_endline
