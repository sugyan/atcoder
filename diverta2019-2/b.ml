open Base;;

let n = Stdlib.read_int () in
let f _ =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun x y -> (x, y))
in
let xy = List.range 0 n |> List.map ~f in
let answer =
  let h = Hashtbl.create (module String) in
  List.cartesian_product xy xy
  |> List.filter ~f:(fun (xy0, xy1) -> Poly.(xy0 <> xy1))
  |> List.iter ~f:(fun ((x0, y0), (x1, y1)) ->
         Printf.sprintf "%d,%d" (x1 - x0) (y1 - y0) |> Hashtbl.incr h);
  Hashtbl.to_alist h |> List.map ~f:snd |> List.fold ~init:0 ~f:max |> ( - ) n
in
answer |> Int.to_string |> Stdlib.print_endline
