open Base;;

let n, a, b =
  Caml.Scanf.sscanf (Caml.read_line ()) "%d %d %d" (fun n a b -> (n, a, b))
in
let answer =
  let rec g n = if n = 0 then 0 else (n % 10) + g (n / 10) in
  let f x = g x |> fun m -> if m >= a && m <= b then x else 0 in
  List.range 1 (n + 1) |> List.sum (module Int) ~f
in
answer |> Int.to_string |> Caml.print_endline
