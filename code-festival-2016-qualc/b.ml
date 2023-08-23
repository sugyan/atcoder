open Base;;

let k, _ = Caml.Scanf.sscanf (Caml.read_line ()) "%d %d" (fun k t -> (k, t)) in
let a =
  Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer = List.fold a ~init:0 ~f:max |> fun m -> m + m - k - 1 |> max 0 in
answer |> Int.to_string |> Caml.print_endline
