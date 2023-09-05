open Base;;

let n, k = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun n k -> (n, k)) in
let a =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let a = List.to_array a in
  let r = List.range 0 n in
  let g ~f = List.sum (module Int) r ~f:(fun i -> List.count r ~f:(f i)) in
  let m0 = g ~f:(fun i j -> a.(i) < a.(j)) in
  let m1 = g ~f:(fun i j -> a.(i) < a.(j) && i > j) in
  ((m1 * k) + (k * (k - 1) / 2 % 1_000_000_007 * m0)) % 1_000_000_007
in
answer |> Int.to_string |> Stdlib.print_endline
