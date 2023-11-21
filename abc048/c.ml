open Base;;

let _, x =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun n x -> (n, x))
in
let a =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let f acc a = if acc + a > x then (x - acc, acc + a - x) else (a, 0) in
  List.folding_map a ~init:0 ~f |> List.sum (module Int) ~f:Fn.id
in
answer |> Int.to_string |> Stdlib.print_endline
