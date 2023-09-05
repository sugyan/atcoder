open Base;;

let s = Stdlib.read_line () in
let t = Stdlib.read_line () in
let answer =
  let len = String.length s in
  List.init len ~f:(fun i -> String.sub (s ^ s) ~pos:i ~len)
  |> List.exists ~f:(Poly.( = ) t)
in
answer |> (function true -> "Yes" | false -> "No") |> Stdlib.print_endline
