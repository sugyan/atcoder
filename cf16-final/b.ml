open Base;;

let n = Stdlib.read_int () in
let answer =
  let rec f i = if i * (i + 1) / 2 >= n then i else f (i + 1) in
  let i = f 1 in
  List.range 1 (i + 1) |> List.filter ~f:(fun x -> x <> (i * (i + 1) / 2) - n)
in
answer |> List.iter ~f:(Fn.compose Stdlib.print_endline Int.to_string)
