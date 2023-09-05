open Base;;

let n, k = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun n k -> (n, k)) in
let answer =
  let rec f x y = if x * y >= k then y else f x (y * 2) in
  List.init n ~f:(fun x -> n * f (x + 1) 1 |> Float.of_int |> ( /. ) 1.0)
  |> List.sum (module Float) ~f:Fn.id
in
answer |> Float.to_string |> Stdlib.print_endline
