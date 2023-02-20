open Base;;

let a, b, c =
  let f a b c = (a, b, c) in
  Caml.Scanf.scanf "%d %d %d" f
in
let rec loop (a, b, c) i =
  let odd x = Caml.( mod ) x 2 = 1 in
  if odd a || odd b || odd c then i
  else if a = b && b = c then -1
  else loop ((b + c) / 2, (a + c) / 2, (a + b) / 2) (i + 1)
in
loop (a, b, c) 0 |> Int.to_string |> Stdio.print_endline
