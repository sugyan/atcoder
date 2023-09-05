open Base;;

let a, b, x =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d %d" (fun a b x -> (a, b, x))
in
let answer = (b / x) - ((a + x - 1) / x) + 1 in
answer |> Int.to_string |> Stdlib.print_endline
