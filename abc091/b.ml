open Base;;

let n = Stdlib.read_int () in
let s = List.range 0 n |> List.map ~f:(fun _ -> Stdlib.read_line ()) in
let m = Stdlib.read_int () in
let t = List.range 0 m |> List.map ~f:(fun _ -> Stdlib.read_line ()) in
let answer =
  let c x = List.count ~f:(String.( = ) x) in
  List.fold s ~init:0 ~f:(fun acc x -> max acc (c x s - c x t))
in
answer |> Int.to_string |> Stdlib.print_endline
