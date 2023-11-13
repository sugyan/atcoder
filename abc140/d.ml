open Base;;

let n, k =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun n k -> (n, k))
in
let s = Stdlib.read_line () in
let answer =
  String.to_list s
  |> List.group ~break:Char.( <> )
  |> List.sum (module Int) ~f:(Fn.compose Int.pred List.length)
  |> ( + ) (k * 2)
  |> min (n - 1)
in
answer |> Int.to_string |> Stdlib.print_endline
