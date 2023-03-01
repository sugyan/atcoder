open Base;;

let x1, y1, x2, y2 =
  Caml.Scanf.sscanf (Caml.read_line ()) "%d %d %d %d" (fun x1 y1 x2 y2 ->
      (x1, y1, x2, y2))
in
let answer =
  let dx = x2 - x1 in
  let dy = y2 - y1 in
  Printf.sprintf "%d %d %d %d" (x2 - dy) (y2 + dx) (x1 - dy) (y1 + dx)
in
answer |> Caml.print_endline
