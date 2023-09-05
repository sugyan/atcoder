open Base;;

let k, _ = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun k t -> (k, t)) in
let a =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer = List.fold a ~init:0 ~f:max |> fun m -> m + m - k - 1 |> max 0 in
answer |> Int.to_string |> Stdlib.print_endline
