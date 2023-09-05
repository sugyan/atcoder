open Base;;

let n, m = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun n m -> (n, m)) in
let answer = min n (m / 2) |> fun c -> c + ((m - (2 * c)) / 4) in
answer |> Int.to_string |> Stdlib.print_endline
