open Base;;

let _ = Stdlib.read_int () in
let a =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  0 :: List.folding_map a ~init:0 ~f:(fun acc x -> acc + x |> fun y -> (y, y))
  |> List.sort ~compare |> List.group ~break:( <> ) |> List.map ~f:List.length
  |> List.sum (module Int) ~f:(fun x -> x * (x - 1) / 2)
in
answer |> Int.to_string |> Stdlib.print_endline
