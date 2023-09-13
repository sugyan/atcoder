open Base;;

let h, w =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun h w -> (h, w))
in
let s = List.range 0 h |> List.map ~f:(fun _ -> Stdlib.read_line ()) in
let answer =
  let a = List.to_array s in
  let bfs p =
    let m = Array.make_matrix ~dimx:h ~dimy:w None in
    let can_move (i, j) =
      0 <= i && i < h && 0 <= j && j < w
      && Option.is_none m.(i).(j)
      && Char.(a.(i).[j] = '.')
    in
    m.(fst p).(snd p) <- Some 0;
    let q = Linked_queue.create () in
    let rec f ((i, j), d) =
      [ (i - 1, j); (i + 1, j); (i, j - 1); (i, j + 1) ]
      |> List.filter ~f:can_move
      |> List.iter ~f:(fun (i, j) ->
             m.(i).(j) <- Some (d + 1);
             Linked_queue.enqueue q ((i, j), d + 1));
      Linked_queue.dequeue q |> Option.iter ~f
    in
    f (p, 0);
    List.cartesian_product (List.range 0 h) (List.range 0 w)
    |> List.filter_map ~f:(fun (i, j) -> m.(i).(j))
    |> List.fold ~init:0 ~f:max
  in
  List.cartesian_product (List.range 0 h) (List.range 0 w)
  |> List.filter ~f:(fun (i, j) -> Char.(a.(i).[j] = '.'))
  |> List.map ~f:bfs |> List.fold ~init:0 ~f:max
in
answer |> Int.to_string |> Stdlib.print_endline
