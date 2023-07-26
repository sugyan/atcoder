open Base;;

let n = Caml.read_int () in
let answer =
  let rec f m = if m = 0 then 0 else m + f (m / 5) in
  if n % 2 = 0 then f (n / 10) else 0
in
answer |> Int.to_string |> Caml.print_endline
