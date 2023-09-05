open Base;;

let s = Stdlib.read_line () in
let answer =
  List.init 26 ~f:(( + ) (Char.to_int 'a'))
  |> List.map ~f:Char.of_int_exn
  |> List.find ~f:(String.contains s |> Fn.non)
in
answer
|> (function Some c -> Char.to_string c | _ -> "None")
|> Stdlib.print_endline
