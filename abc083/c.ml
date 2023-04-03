open Base;;

let x, y = Caml.Scanf.sscanf (Caml.read_line ()) "%d %d" (fun x y -> (x, y)) in
let answer =
  let rec f n = if n > y then 0 else 1 + f (n * 2) in
  f x
in
answer |> Int.to_string |> Caml.print_endline
