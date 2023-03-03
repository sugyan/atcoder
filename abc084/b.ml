open Base;;

let a, _ = Caml.Scanf.sscanf (Caml.read_line ()) "%d %d" (fun a b -> (a, b)) in
let s = Caml.read_line () in
let answer =
  String.to_list s
  |> List.for_alli ~f:(fun i -> (if i = a then Char.( = ) else Char.( <> )) '-')
in
answer |> (function true -> "Yes" | false -> "No") |> Caml.print_endline
