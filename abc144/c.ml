open Base;;

let n = Caml.read_int () in
let answer =
  let rec loop i acc =
    if i * i > n then acc else loop (i + 1) (if n % i = 0 then i else acc)
  in
  loop 1 1 |> fun x -> x + (n / x) - 2
in
answer |> Int.to_string |> Caml.print_endline
