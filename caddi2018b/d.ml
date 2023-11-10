open Base;;

let n = Stdlib.read_int () in
let a = List.range 0 n |> List.map ~f:(fun _ -> Stdlib.read_int ()) in
let answer =
  if List.map a ~f:(Fn.flip ( % ) 2) |> List.for_all ~f:(( = ) 0) then `second
  else `first
in
answer
|> (function `first -> "first" | `second -> "second")
|> Stdlib.print_endline
