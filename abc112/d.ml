open Base;;

let n, m =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun n m -> (n, m))
in
let answer =
  let rec f acc i =
    if i * i > m then acc
    else if m % i = 0 then f (i :: (m / i) :: acc) (i + 1)
    else f acc (i + 1)
  in
  f [] 1 |> List.filter ~f:(fun i -> m / i >= n) |> List.fold ~init:1 ~f:max
in
answer |> Int.to_string |> Stdlib.print_endline
