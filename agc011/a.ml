open Base;;

let n, c, k =
  Caml.Scanf.sscanf (Caml.read_line ()) "%d %d %d" (fun n c k -> (n, c, k))
in
let t = List.range 0 n |> List.map ~f:(fun _ -> Caml.read_int ()) in
let answer =
  let f ((i, t0), acc) x =
    if i = 0 || x > t0 + k then ((c - 1, x), acc + 1) else ((i - 1, t0), acc)
  in
  List.sort t ~compare |> List.fold ~init:((0, 0), 0) ~f |> snd
in
answer |> Int.to_string |> Caml.print_endline
