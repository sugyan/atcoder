open Base;;

let s = Stdlib.read_int () in
let answer =
  let m = 1_000_000_000 in
  let x3 = m - (((s - 1) % m) + 1) in
  let y3 = ((s - 1) / m) + 1 in
  (0, 0, m, 1, x3, y3)
in
answer
|> (fun (x1, y1, x2, y2, x3, y3) ->
     Printf.sprintf "%d %d %d %d %d %d" x1 y1 x2 y2 x3 y3)
|> Stdlib.print_endline
