open Core;;

let n, m, k =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d %d" (fun n m k -> (n, m, k))
in
let ab, cd =
  let f _ =
    Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun x y -> (x, y))
  in
  let ab = List.range 0 m |> List.map ~f in
  let cd = List.range 0 k |> List.map ~f in
  (ab, cd)
in
let answer =
  let classes = Array.init n ~f:Union_find.create in
  let graph = Array.create ~len:n [] in
  let sizes = Array.create ~len:n 0 in
  List.iter ab ~f:(fun (a, b) ->
      Union_find.union classes.(a - 1) classes.(b - 1));
  List.concat [ ab; cd ]
  |> List.iter ~f:(fun (x, y) ->
         graph.(x - 1) <- (y - 1) :: graph.(x - 1);
         graph.(y - 1) <- (x - 1) :: graph.(y - 1));
  Array.map classes ~f:Union_find.get
  |> Array.iter ~f:(fun i -> sizes.(i) <- sizes.(i) + 1);
  let f i =
    List.map graph.(i) ~f:(Array.get classes)
    |> List.count ~f:(Union_find.same_class classes.(i))
    |> ( - ) (sizes.(Union_find.get classes.(i)) - 1)
  in
  List.init n ~f
in
answer |> List.map ~f:Int.to_string |> String.concat ~sep:" "
|> Stdlib.print_endline
