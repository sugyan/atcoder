open Base;;

let n = Caml.read_int () in
let answer =
  let a = Array.create ~len:1000 1 in
  let rec f i m =
    if m = 1 then ()
    else if m % i = 0 then (
      a.(i) <- a.(i) + 1;
      f i (m / i))
    else f (i + 1) m
  in
  List.range 2 (n + 1) |> List.iter ~f:(f 2);
  Array.fold a ~init:1 ~f:(fun acc x -> acc * x % 1_000_000_007)
in
answer |> Int.to_string |> Caml.print_endline
