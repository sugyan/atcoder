open Base;;

let h, w =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun h w -> (h, w))
in
let a = List.range 0 h |> List.map ~f:(fun _ -> Stdlib.read_line ()) in
let answer =
  let a = List.to_array a in
  let m = Array.make_matrix ~dimx:h ~dimy:w None in
  let q = Queue.create () in
  let f (i, j) c =
    m.(i).(j) <- Some c;
    Queue.enqueue q ((i, j), c)
  in
  List.cartesian_product (List.range 0 h) (List.range 0 w)
  |> List.iter ~f:(fun (i, j) -> if Char.(a.(i).[j] = '#') then f (i, j) 0);
  let rec bfs acc =
    Queue.dequeue q |> function
    | Some ((i, j), c) ->
        [ (i - 1, j); (i + 1, j); (i, j - 1); (i, j + 1) ]
        |> List.filter ~f:(fun (i, j) ->
               0 <= i && i < h && 0 <= j && j < w && Option.is_none m.(i).(j))
        |> List.iter ~f:(fun (i, j) -> f (i, j) (c + 1));
        bfs c
    | None -> acc
  in
  bfs 0
in
answer |> Int.to_string |> Stdlib.print_endline
