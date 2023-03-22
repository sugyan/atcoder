open Base;;

let s = Caml.read_line () in
let answer = String.(rindex_exn s 'Z' - index_exn s 'A') + 1 in
answer |> Int.to_string |> Caml.print_endline
