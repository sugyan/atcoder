open Base;;

let a, _ = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun a b -> (a, b)) in
let s = Stdlib.read_line () in
let answer =
  String.to_list s
  |> List.for_alli ~f:(fun i -> (if i = a then Char.( = ) else Char.( <> )) '-')
in
answer |> (function true -> "Yes" | false -> "No") |> Stdlib.print_endline
