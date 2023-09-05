open Base;;

let d, n = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun d n -> (d, n)) in
let answer = (100 ** d) * (n + (n / 100)) in
answer |> Int.to_string |> Stdlib.print_endline
