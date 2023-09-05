open Base;;

let c =
  let f _ =
    Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d %d" (fun c1 c2 c3 ->
        [ c1; c2; c3 ])
  in
  List.range 0 3 |> List.map ~f
in
let answer =
  let c = List.map c ~f:List.to_array |> List.to_array in
  [
    List.init 3 ~f:(fun i -> (c.(i).(1) - c.(i).(0), c.(i).(2) - c.(i).(1)));
    List.init 3 ~f:(fun i -> (c.(1).(i) - c.(0).(i), c.(2).(i) - c.(1).(i)));
  ]
  |> List.for_all ~f:(fun l ->
         List.dedup_and_sort l ~compare:Poly.compare |> List.length |> ( = ) 1)
in
answer |> (function true -> "Yes" | false -> "No") |> Stdlib.print_endline
