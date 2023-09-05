open Base;;

let x = Stdlib.read_int () in
let answer =
  List.cartesian_product (List.range 1 32) (List.range 2 10)
  |> List.map ~f:(fun (a, b) -> a ** b)
  |> List.filter ~f:(( >= ) x)
  |> List.fold ~init:0 ~f:max
in
answer |> Int.to_string |> Stdlib.print_endline
