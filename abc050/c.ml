open Base;;

let n = Stdlib.read_int () in
let a =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  List.init n ~f:(fun i -> n - 1 - (i / 2 * 2))
  |> Poly.equal (List.sort a ~compare:descending)
  |> function
  | true -> Fn.apply_n_times ~n:(n / 2) (fun x -> x * 2 % 1_000_000_007) 1
  | false -> 0
in
answer |> Int.to_string |> Stdlib.print_endline
