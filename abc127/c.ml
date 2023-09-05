open Base;;

let n, m = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun n m -> (n, m)) in
let lr =
  List.init m ~f:(fun _ ->
      Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun l r -> (l, r)))
in
let answer =
  1
  - (List.map lr ~f:fst |> List.fold ~init:0 ~f:max)
  + (List.map lr ~f:snd |> List.fold ~init:n ~f:min)
  |> max 0
in
answer |> Int.to_string |> Stdlib.print_endline
