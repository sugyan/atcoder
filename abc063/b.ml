open Base;;

let s = Caml.read_line () in
let answer = String.to_list s |> List.contains_dup ~compare:Char.compare in
answer |> (function true -> "no" | false -> "yes") |> Caml.print_endline
