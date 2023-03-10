open Base;;

let a, b, k =
  Caml.Scanf.sscanf (Caml.read_line ()) "%d %d %d %d" (fun a b _ k -> (a, b, k))
in
let answer = if k % 2 = 0 then a - b else b - a in
answer |> Int.to_string |> Caml.print_endline
