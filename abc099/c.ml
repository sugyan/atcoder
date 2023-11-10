open Base;;

let n = Stdlib.read_int () in
let answer =
  let rec f m c = if m = 0 then 0 else (m % c) + f (m / c) c in
  List.(init (n + 1) ~f:(fun i -> f i 6 + f (n - i) 9) |> fold ~init:n ~f:min)
in
answer |> Int.to_string |> Stdlib.print_endline
