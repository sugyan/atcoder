open Base;;

let f _ =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d %d" (fun x y z -> (x, y, z))
in
let w, h, n = f () in
let xya = List.range 0 n |> List.map ~f in
let answer =
  List.fold xya ~init:(0, 0, w, h) ~f:(fun (xmin, ymin, xmax, ymax) (x, y, a) ->
      match a with
      | 1 -> (max x xmin, ymin, xmax, ymax)
      | 2 -> (xmin, ymin, min x xmax, ymax)
      | 3 -> (xmin, max y ymin, xmax, ymax)
      | 4 -> (xmin, ymin, xmax, min y ymax)
      | _ -> assert false)
  |> fun (xmin, ymin, xmax, ymax) -> max 0 (xmax - xmin) * (ymax - ymin)
in
answer |> Int.to_string |> Stdlib.print_endline
