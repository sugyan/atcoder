open Base;;

let n, m = Caml.Scanf.sscanf (Caml.read_line ()) "%d %d" (fun n m -> (n, m)) in
let ab =
  List.range 0 m
  |> List.map ~f:(fun _ ->
         Caml.Scanf.sscanf (Caml.read_line ()) "%d %d" (fun a b -> (a, b)))
in
let answer =
  let a = List.filter ab ~f:(fun (a, _) -> a = 1) |> List.map ~f:snd in
  let b = List.filter ab ~f:(fun (_, b) -> b = n) |> List.map ~f:fst in
  List.append a b |> List.contains_dup ~compare
in
answer
|> (function true -> "POSSIBLE" | false -> "IMPOSSIBLE")
|> Caml.print_endline
