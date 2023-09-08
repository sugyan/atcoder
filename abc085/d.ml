open Base;;

let f _ =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun x y -> (x, y))
in
let n, h = f () in
let ab = List.range 0 n |> List.map ~f in
let answer =
  let m = List.map ab ~f:fst |> List.fold ~init:0 ~f:max in
  let c x = if x > 0 then ((x - 1) / m) + 1 else 0 in
  let f i acc x = (acc + x, i + 1 + c (h - acc - x)) in
  List.map ab ~f:snd
  |> List.sort ~compare:descending
  |> List.folding_mapi ~init:0 ~f
  |> List.fold ~init:(c h) ~f:min
in
answer |> Int.to_string |> Stdlib.print_endline
