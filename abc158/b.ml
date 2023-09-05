open Base;;

let n, a, b =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d %d" (fun n a b -> (n, a, b))
in
let answer = (n / (a + b) * a) + min a (n % (a + b)) in
answer |> Int.to_string |> Stdlib.print_endline
