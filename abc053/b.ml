open Base;;

let s = Stdlib.read_line () in
let answer = String.(rindex_exn s 'Z' - index_exn s 'A') + 1 in
answer |> Int.to_string |> Stdlib.print_endline
