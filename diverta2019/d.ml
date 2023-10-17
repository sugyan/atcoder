open Base;;

let n = Stdlib.read_int () in
let answer =
  let rec f m acc =
    if (n - m) / m > m then
      (acc + if (n - (n / m)) % (n / m) = 0 then (n - m) / m else 0) |> f (m + 1)
    else acc
  in
  f 1 0
in
answer |> Int.to_string |> Stdlib.print_endline
