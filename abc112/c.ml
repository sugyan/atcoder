open Base;;

let n = Stdlib.read_int () in
let xyh =
  let f _ =
    Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d %d" (fun x y h ->
        (x, y, h))
  in
  List.range 0 n |> List.map ~f
in
let answer =
  let x0, y0, h0 = List.find_exn xyh ~f:(fun (_, _, h) -> h > 0) in
  let f (cx, cy) =
    let ch = h0 + abs (cx - x0) + abs (cy - y0) in
    let g (x, y, h) = ch - abs (cx - x) - abs (cy - y) |> max 0 = h in
    if List.for_all xyh ~f:g then Some (cx, cy, ch) else None
  in
  List.cartesian_product (List.range 0 101) (List.range 0 101)
  |> List.find_map_exn ~f
in
answer
|> (fun (x, y, h) -> Printf.sprintf "%d %d %d" x y h)
|> Stdlib.print_endline
