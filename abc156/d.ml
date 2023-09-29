open Base;;

let n, a, b =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d %d" (fun n a b -> (n, a, b))
in
let answer =
  let p = 1_000_000_007 in
  let rec pow x n =
    if n = 0 then 1
    else if n % 2 = 0 then pow x (n / 2) |> fun y -> y * y % p
    else x * pow x (n - 1) % p
  in
  let f lo hi =
    List.range lo (hi + 1) |> List.fold ~init:1 ~f:(fun acc x -> acc * x % p)
  in
  let nck n k = f (n - k + 1) n * pow (f 1 k) (p - 2) % p in
  (pow 2 n - 1 - nck n a - nck n b) % p
in
answer |> Int.to_string |> Stdlib.print_endline
