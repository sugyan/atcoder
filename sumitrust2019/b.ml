open Base;;

let n = Stdlib.read_int () in
let f i = i * 108 / 100 = n in
List.range 1 50000 |> List.find ~f
|> (function Some i -> Int.to_string i | None -> ":(")
|> Stdlib.print_endline
