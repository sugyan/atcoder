open Base;;

let a, b, x =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d %d" (fun a b x -> (a, b, x))
in
let answer =
  (if x < a * a * b / 2 then
   Stdlib.atan2 (Float.of_int (b * b * a)) (Float.of_int (2 * x))
  else
    Stdlib.atan2 (Float.of_int (2 * ((a * a * b) - x))) (Float.of_int (a * a * a)))
  |> ( *. ) 180. |> Fn.flip ( /. ) Float.pi
in
answer |> Float.to_string |> Stdlib.print_endline
