open Base;;

let w, h, x, y =
  Caml.Scanf.sscanf (Caml.read_line ()) "%d %d %d %d" (fun w h x y ->
      (w, h, x, y))
in
let answer = (Float.of_int (w * h) /. 2.0, x * 2 = w && y * 2 = h) in
answer
|> (fun (a, b) -> Printf.sprintf "%f %d" a (Bool.to_int b))
|> Caml.print_endline
