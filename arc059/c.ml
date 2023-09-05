open Base;;

let _ = Stdlib.read_int () in
let a =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let f m = List.sum (module Int) a ~f:(fun x -> (x - m) ** 2) in
  List.range (-100) 101 |> List.map ~f |> List.fold ~init:(f 0) ~f:min
in
answer |> Int.to_string |> Stdlib.print_endline
