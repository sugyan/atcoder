open Base;;

let n, m = Caml.Scanf.sscanf (Caml.read_line ()) "%d %d" (fun n m -> (n, m)) in
let lr =
  List.init m ~f:(fun _ ->
      Caml.Scanf.sscanf (Caml.read_line ()) "%d %d" (fun l r -> (l, r)))
in
let answer =
  1
  - (List.map lr ~f:fst |> List.fold ~init:0 ~f:max)
  + (List.map lr ~f:snd |> List.fold ~init:n ~f:min)
  |> max 0
in
answer |> Int.to_string |> Caml.print_endline
