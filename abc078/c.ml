open Base;;

let n, m = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun n m -> (n, m)) in
let answer = ((n + (m * 18)) * 100) lsl m in
answer |> Int.to_string |> Stdlib.print_endline
