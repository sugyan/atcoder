open Base;;

let n, k =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun n k -> (n, k))
in
let answer =
  let p = 1_000_000_007 in
  let rec pow x y =
    if y = 0 then 1
    else if y % 2 = 0 then pow (x * x % p) (y / 2)
    else x * pow (x * x % p) (y / 2) % p
  in
  let f i j =
    List.range i (j + 1) |> List.fold ~init:1 ~f:(fun acc x -> acc * x % p)
  in
  let nck n k = f (n - k + 1) n * pow (f 1 k) (p - 2) % p in
  List.init k ~f:(fun i -> nck (k - 1) i * nck (n - k + 1) (i + 1) % p)
in
answer |> List.map ~f:Int.to_string |> List.iter ~f:Stdlib.print_endline
