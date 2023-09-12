open Base;;

let h, w, _ =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d %d" (fun h w k -> (h, w, k))
in
let s = List.range 0 h |> List.map ~f:(fun _ -> Stdlib.read_line ()) in
let answer =
  let a = List.to_array s in
  let m = Array.make_matrix ~dimx:h ~dimy:w 0 in
  let not_empty s = String.(s <> make w '.') in
  let split_indices len l =
    (List.hd_exn l |> min 0) :: List.tl_exn l
    |> List.rev
    |> List.folding_map ~init:len ~f:(fun acc x -> (x, List.range x acc))
    |> List.rev
  in
  let f i range_x =
    let g j range_y =
      List.cartesian_product range_x range_y
      |> List.iter ~f:(fun (x, y) -> m.(x).(y) <- j);
      j + 1
    in
    range_x
    |> List.map ~f:(Array.get a)
    |> List.find_exn ~f:not_empty |> String.to_list
    |> List.filter_mapi ~f:(fun i c -> if Char.(c = '#') then Some i else None)
    |> split_indices w |> List.fold ~init:i ~f:g
  in
  List.filter_mapi s ~f:(fun i row -> if not_empty row then Some i else None)
  |> split_indices h |> List.fold ~init:1 ~f |> ignore;
  Array.map m ~f:Array.to_list |> Array.to_list
in
let f row =
  List.map row ~f:Int.to_string
  |> String.concat ~sep:" " |> Stdlib.print_endline
in
answer |> List.iter ~f
