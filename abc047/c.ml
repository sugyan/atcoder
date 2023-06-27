open Base;;

let s = Caml.read_line () in
let answer =
  (String.to_list s |> List.group ~break:Char.( <> ) |> List.length) - 1
in
answer |> Int.to_string |> Caml.print_endline
