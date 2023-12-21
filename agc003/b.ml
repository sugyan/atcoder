open Base;;

let n = Stdlib.read_int () in
let a = List.range 0 n |> List.map ~f:(fun _ -> Stdlib.read_int ()) in
let answer =
  let f l = List.fold l ~init:0 ~f:( + ) / 2 in
  List.group a ~break:(fun x _ -> x = 0) |> List.sum (module Int) ~f
in
answer |> Int.to_string |> Stdlib.print_endline
