open Base;;

let a, b, c, d =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d %d %d" (fun a b c d ->
      (a, b, c, d))
in
let answer =
  let rec gcd m n = if n = 0 then m else gcd n (m % n) in
  let f x = (b / x) - ((a - 1) / x) in
  b - a + 1 - f c - f d + f (c * d / gcd c d)
in
answer |> Int.to_string |> Stdlib.print_endline
