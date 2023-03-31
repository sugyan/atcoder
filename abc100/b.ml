open Base;;

let d, n = Caml.Scanf.sscanf (Caml.read_line ()) "%d %d" (fun d n -> (d, n)) in
let answer = (100 ** d) * (n + (n / 100)) in
answer |> Int.to_string |> Caml.print_endline
