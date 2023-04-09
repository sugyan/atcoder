open Base;;

let n, k = Caml.Scanf.sscanf (Caml.read_line ()) "%d %d" (fun n k -> (n, k)) in
let answer = k * ((k - 1) ** (n - 1)) in
answer |> Int.to_string |> Caml.print_endline
