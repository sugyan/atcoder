open Base;;

let s = Stdlib.read_line () in
let k = Stdlib.read_int () in
let answer =
  String.lfindi s ~f:(fun _ c -> Char.(c <> '1')) |> function
  | Some i when i < k -> s.[i]
  | _ -> '1'
in
answer |> Char.to_string |> Stdlib.print_endline
