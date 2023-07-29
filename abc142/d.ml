open Base;;

let a, b = Caml.Scanf.sscanf (Caml.read_line ()) "%d %d" (fun a b -> (a, b)) in
let answer =
  let rec gcd x y = if y = 0 then x else gcd y (x % y) in
  let rec f i n acc =
    if i * i > n then n :: acc
    else if n % i = 0 then f i (n / i) (i :: acc)
    else f (i + 1) n acc
  in
  f 2 (gcd a b) [ 1 ] |> List.dedup_and_sort ~compare |> List.length
in
answer |> Int.to_string |> Caml.print_endline
