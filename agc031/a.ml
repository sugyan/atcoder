open Base;;

let _ = Caml.read_int () in
let s = Caml.read_line () in
let answer =
  let h = Hashtbl.create (module Char) ~size:26 in
  String.iter s ~f:(Hashtbl.incr h);
  let f acc x = ((acc * (x + 1)) + x) % 1_000_000_007 in
  Hashtbl.data h |> List.fold ~init:0 ~f
in
answer |> Int.to_string |> Caml.print_endline
