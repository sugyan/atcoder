open Base;;

let n, k = Caml.Scanf.sscanf (Caml.read_line ()) "%d %d" (fun n k -> (n, k)) in
let answer = if n = k then 1 else ((n - k - 1) / (k - 1)) + 2 in
answer |> Int.to_string |> Caml.print_endline
