open Base;;

let n = Caml.read_int () in
let t = List.range 0 n |> List.map ~f:(fun _ -> Caml.read_int ()) in
let answer =
  let rec gcd a b = if b = 0 then a else gcd b (a % b) in
  let lcm a b = a / gcd a b * b in
  List.fold t ~init:1 ~f:lcm
in
answer |> Int.to_string |> Caml.print_endline
