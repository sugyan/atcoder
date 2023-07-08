open Base;;

let a, b, x =
  Caml.Scanf.sscanf (Caml.read_line ()) "%d %d %d" (fun a b x -> (a, b, x))
in
let answer =
  (if x < a * a * b / 2 then
   Caml.atan2 (Float.of_int (b * b * a)) (Float.of_int (2 * x))
  else
    Caml.atan2 (Float.of_int (2 * ((a * a * b) - x))) (Float.of_int (a * a * a)))
  |> ( *. ) 180. |> Fn.flip ( /. ) Float.pi
in
answer |> Float.to_string |> Caml.print_endline
