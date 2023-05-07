open Base;;

let n, m = Caml.Scanf.sscanf (Caml.read_line ()) "%d %d" (fun n m -> (n, m)) in
let answer = abs (n - 2) * abs (m - 2) in
answer |> Int.to_string |> Caml.print_endline
