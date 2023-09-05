open Base;;

let n = Stdlib.read_int () in
let answer =
  let rec f i acc =
    if i * i > n then acc else f (i + 1) (if n % i = 0 then i :: acc else acc)
  in
  let rec g m = if m = 0 then 0 else 1 + g (m / 10) in
  f 1 []
  |> List.fold ~init:10 ~f:(fun acc i -> max (g i) (g (n / i)) |> min acc)
in
answer |> Int.to_string |> Stdlib.print_endline
