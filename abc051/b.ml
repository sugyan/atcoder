open Base;;

let k, s = Caml.Scanf.sscanf (Caml.read_line ()) "%d %d" (fun k s -> (k, s)) in
let answer =
  let f x = s - x - k |> abs |> ( - ) (k + 1) |> max 0 in
  List.init (k + 1) ~f |> List.fold ~init:0 ~f:( + )
in
answer |> Int.to_string |> Caml.print_endline
