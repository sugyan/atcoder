open Base;;

let n, m = Caml.Scanf.sscanf (Caml.read_line ()) "%d %d" (fun n m -> (n, m)) in
let answer = ((n + (m * 18)) * 100) lsl m in
answer |> Int.to_string |> Caml.print_endline
