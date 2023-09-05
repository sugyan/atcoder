open Base;;

let n, m = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun n m -> (n, m)) in
let answer = abs (n - 2) * abs (m - 2) in
answer |> Int.to_string |> Stdlib.print_endline
