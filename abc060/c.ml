open Base;;

let t = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun _ t -> t) in
let ts =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  List.fold ts ~init:(t, 0) ~f:(fun (sum, p) x -> (sum + min t (x - p), x))
  |> fst
in
answer |> Int.to_string |> Stdlib.print_endline
