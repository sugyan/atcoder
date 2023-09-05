open Base;;

let n, m = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun n m -> (n, m)) in
let ab =
  List.range 0 m
  |> List.map ~f:(fun _ ->
         Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun a b -> (a, b)))
in
let answer =
  let a = List.filter ab ~f:(fun (a, _) -> a = 1) |> List.map ~f:snd in
  let b = List.filter ab ~f:(fun (_, b) -> b = n) |> List.map ~f:fst in
  List.append a b |> List.contains_dup ~compare
in
answer
|> (function true -> "POSSIBLE" | false -> "IMPOSSIBLE")
|> Stdlib.print_endline
