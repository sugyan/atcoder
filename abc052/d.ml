open Base;;

let n, a, b =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d %d" (fun n a b -> (n, a, b))
in
let x =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let x = List.to_array x in
  List.range 0 (n - 1)
  |> List.sum (module Int) ~f:(fun i -> (x.(i + 1) - x.(i)) * a |> min b)
in
answer |> Int.to_string |> Stdlib.print_endline
