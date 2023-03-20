open Base;;

let s = Caml.read_line () in
let t = Caml.read_line () in
let answer =
  let len = String.length s in
  List.init len ~f:(fun i -> String.sub (s ^ s) ~pos:i ~len)
  |> List.exists ~f:(Poly.( = ) t)
in
answer |> (function true -> "Yes" | false -> "No") |> Caml.print_endline
