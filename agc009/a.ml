open Base;;

let n = Caml.read_int () in
let ab =
  List.range 0 n
  |> List.map ~f:(fun _ ->
         Caml.Scanf.sscanf (Caml.read_line ()) "%d %d " (fun a b -> (a, b)))
in
let answer =
  List.fold_right ab ~init:0 ~f:(fun (a, b) acc ->
      acc - 1 + b - ((acc - 1 + a) % b))
in
answer |> Int.to_string |> Caml.print_endline
