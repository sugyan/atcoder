open Base;;

let n = Caml.read_int () in
let xy =
  let f _ = Caml.Scanf.sscanf (Caml.read_line ()) "%f %f" (fun x y -> (x, y)) in
  List.range 0 n |> List.map ~f
in
let answer =
  List.cartesian_product xy xy
  |> List.map ~f:(fun ((xi, yi), (xj, yj)) ->
         ((xi -. xj) **. 2.) +. ((yi -. yj) **. 2.) |> Float.sqrt)
  |> List.sum (module Float) ~f:(Fn.flip ( /. ) (Float.of_int n))
in
answer |> Float.to_string |> Caml.print_endline
