open Base;;

let s = Stdlib.read_line () in
let answer = String.to_list s |> List.contains_dup ~compare:Char.compare in
answer |> (function true -> "no" | false -> "yes") |> Stdlib.print_endline
