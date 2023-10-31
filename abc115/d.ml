open Base;;

let n, x =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun n x -> (n, x))
in
let answer =
  let rec f n x =
    if n = 0 then Bool.to_int (x > 0)
    else if x < (2 lsl n) - 1 then f (n - 1) (x - 1)
    else (1 lsl n) + f (n - 1) (x + 1 - (2 lsl n))
  in
  f n x
in
answer |> Int.to_string |> Stdlib.print_endline
