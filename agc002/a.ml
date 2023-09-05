open Base;;

let a, b = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun a b -> (a, b)) in
let answer =
  if a * b < 0 then "Zero"
  else if b < 0 && (b - a) % 2 = 0 then "Negative"
  else "Positive"
in
answer |> Stdlib.print_endline
