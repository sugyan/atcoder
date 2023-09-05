open Base;;

let n = Stdlib.read_int () in
let sp =
  List.range 0 n
  |> List.map ~f:(fun _ ->
         Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%s %d" (fun s p -> (s, -p)))
in
let answer =
  List.mapi sp ~f:(fun i e -> (e, i + 1))
  |> List.sort ~compare:Poly.compare
  |> List.map ~f:snd
in
answer |> List.map ~f:Int.to_string |> List.iter ~f:Stdlib.print_endline
