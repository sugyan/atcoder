open Base;;

let a, b, k =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d %d" (fun a b k -> (a, b, k))
in
let answer =
  List.range 0 k
  |> List.map ~f:(fun i -> [ a + i; b - i ])
  |> List.concat
  |> List.filter ~f:(fun x -> x >= a && x <= b)
  |> List.dedup_and_sort ~compare
in
answer |> List.iter ~f:(Fn.compose Stdlib.print_endline Int.to_string)
