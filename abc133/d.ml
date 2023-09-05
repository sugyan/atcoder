open Base;;

let _ = Caml.read_int () in
let a =
  Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let init = List.fold a ~init:0 ~f:(fun acc x -> x + x - acc) / 2 in
  List.folding_map a ~init ~f:(fun acc x -> (x + x - acc, acc))
in
answer |> List.map ~f:Int.to_string |> String.concat ~sep:" "
|> Caml.print_endline
