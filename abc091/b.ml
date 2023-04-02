open Base;;

let n = Caml.read_int () in
let s = List.range 0 n |> List.map ~f:(fun _ -> Caml.read_line ()) in
let m = Caml.read_int () in
let t = List.range 0 m |> List.map ~f:(fun _ -> Caml.read_line ()) in
let answer =
  let c x = List.count ~f:(String.( = ) x) in
  List.fold s ~init:0 ~f:(fun acc x -> max acc (c x s - c x t))
in
answer |> Int.to_string |> Caml.print_endline
