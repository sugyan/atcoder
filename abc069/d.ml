open Base;;

let _, w = Caml.Scanf.sscanf (Caml.read_line ()) "%d %d" (fun h w -> (h, w)) in
let _ = Caml.read_int () in
let a =
  Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  List.mapi a ~f:(fun i j -> List.init j ~f:(Fn.const (i + 1)))
  |> List.concat |> List.chunks_of ~length:w
  |> List.mapi ~f:(fun i l -> if i % 2 = 0 then l else List.rev l)
in
List.iter answer ~f:(fun l ->
    List.map l ~f:Int.to_string |> String.concat ~sep:" " |> Caml.print_endline)
