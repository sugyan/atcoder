open Base;;

let n = Stdlib.read_int () in
let xl =
  let f _ = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun x l -> (x, l)) in
  List.range 0 n |> List.map ~f
in
let answer =
  List.map xl ~f:(fun (x, l) -> (x + l, x - l))
  |> List.sort ~compare:Poly.compare
  |> List.fold ~init:(0, Int.min_value) ~f:(fun (acc, pt) (t, s) ->
         if pt > s then (acc, pt) else (1 + acc, t))
  |> fst
in
answer |> Int.to_string |> Stdlib.print_endline
