open Base;;

let a, b, x =
  Caml.Scanf.sscanf (Caml.read_line ()) "%d %d %d" (fun a b x -> (a, b, x))
in
let answer = (b / x) - ((a + x - 1) / x) + 1 in
answer |> Int.to_string |> Caml.print_endline
