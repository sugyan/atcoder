open Base;;

let n, m = Caml.Scanf.sscanf (Caml.read_line ()) "%d %d" (fun n m -> (n, m)) in
let s = Caml.read_line () in
let t = Caml.read_line () in
let answer =
  let rec gcd a b = if b = 0 then a else gcd b (a % b) in
  let g = gcd n m in
  List.range 0 g
  |> List.for_all ~f:(fun i -> Char.(s.[i * (n / g)] = t.[i * (m / g)]))
  |> function
  | true -> n * m / g
  | false -> -1
in
answer |> Int.to_string |> Caml.print_endline
