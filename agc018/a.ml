open Base;;

let _, k = Caml.Scanf.sscanf (Caml.read_line ()) "%d %d" (fun n k -> (n, k)) in
let a =
  Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let rec gcd x y = if y = 0 then x else gcd y (x % y) in
  let g = List.fold a ~init:0 ~f:gcd in
  List.fold a ~init:0 ~f:max |> fun x -> x >= k && k % g = 0
in
answer
|> (function true -> "POSSIBLE" | false -> "IMPOSSIBLE")
|> Caml.print_endline
