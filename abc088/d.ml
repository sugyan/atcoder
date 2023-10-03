open Base;;

let h, w =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun h w -> (h, w))
in
let s = List.range 0 h |> List.map ~f:(fun _ -> Stdlib.read_line ()) in
let answer =
  let c = List.sum (module Int) ~f:(String.count ~f:(Char.( = ) '#')) s in
  let s = Array.of_list s in
  let a = Array.make_matrix ~dimx:h ~dimy:w None in
  let q = Linked_queue.create () in
  a.(0).(0) <- Some 1;
  let rec bfs ((i, j), d) =
    [ (i - 1, j); (i + 1, j); (i, j - 1); (i, j + 1) ]
    |> List.filter ~f:(fun (i, j) -> 0 <= i && i < h && 0 <= j && j < w)
    |> List.filter ~f:(fun (i, j) ->
           Char.(s.(i).[j] = '.') && Option.is_none a.(i).(j))
    |> List.iter ~f:(fun (i, j) ->
           a.(i).(j) <- Some (d + 1);
           Linked_queue.enqueue q ((i, j), d + 1));
    Linked_queue.dequeue q |> Option.iter ~f:bfs
  in
  bfs ((0, 0), 1);
  a.(h - 1).(w - 1) |> function None -> -1 | Some x -> (h * w) - x - c
in
answer |> Int.to_string |> Stdlib.print_endline
