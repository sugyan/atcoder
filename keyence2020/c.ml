open Base;;

let n, k, s =
  Caml.Scanf.sscanf (Caml.read_line ()) "%d %d %d" (fun n k s -> (n, k, s))
in
let answer =
  List.init k ~f:(Fn.const s)
  @ List.init (n - k) ~f:(Fn.const ((s % 1000000000) + 1))
in
answer |> List.map ~f:Int.to_string |> String.concat ~sep:" "
|> Caml.print_endline
