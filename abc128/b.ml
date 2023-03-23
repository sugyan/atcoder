open Base;;

let n = Caml.read_int () in
let sp =
  List.range 0 n
  |> List.map ~f:(fun _ ->
         Caml.Scanf.sscanf (Caml.read_line ()) "%s %d" (fun s p -> (s, -p)))
in
let answer =
  List.mapi sp ~f:(fun i e -> (e, i + 1))
  |> List.sort ~compare:Poly.compare
  |> List.map ~f:snd
in
answer |> List.map ~f:Int.to_string |> List.iter ~f:Caml.print_endline
