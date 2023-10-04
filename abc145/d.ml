open Base;;

let x, y =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun x y -> (x, y))
in
let answer =
  let p = 1_000_000_007 in
  let rec pow n x =
    if x = 0 then 1
    else if x % 2 = 0 then pow n (x / 2) |> fun y -> y * y % p
    else n * pow n (x - 1) % p
  in
  let f lo hi =
    List.range lo (hi + 1) |> List.fold ~init:1 ~f:(fun acc x -> acc * x % p)
  in
  let nck n k = f (n - k + 1) n * pow (f 1 k) (p - 2) % p in
  if x * 2 < y || y * 2 < x || (x + y) % 3 > 0 then 0
  else (x - ((x + y) / 3), y - ((x + y) / 3)) |> fun (x, y) -> nck (x + y) x
in
answer |> Int.to_string |> Stdlib.print_endline
