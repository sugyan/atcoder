open Base;;

let n, m = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun n m -> (n, m)) in
let ab =
  List.range 0 n
  |> List.map ~f:(fun _ ->
         Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun a b -> (a, b)))
in
let answer =
  List.sort ab ~compare:Poly.compare
  |> List.fold ~init:(0, m) ~f:(fun (sum, m) (a, b) ->
         (sum + (a * min b m), m - min b m))
  |> fst
in
answer |> Int.to_string |> Stdlib.print_endline
