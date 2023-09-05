open Base;;

let s = Stdlib.read_line () in
let answer =
  (String.to_list s |> List.group ~break:Char.( <> ) |> List.length) - 1
in
answer |> Int.to_string |> Stdlib.print_endline
