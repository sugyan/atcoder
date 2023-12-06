open Base;;

let n = Stdlib.read_int () in
let uvw =
  let f _ =
    Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d %d" (fun u v w ->
        (u, v, w))
  in
  List.range 0 (n - 1) |> List.map ~f
in
let answer =
  let g = Array.create ~len:n [] in
  List.iter uvw ~f:(fun (u, v, w) ->
      g.(u - 1) <- (v - 1, w % 2) :: g.(u - 1);
      g.(v - 1) <- (u - 1, w % 2) :: g.(v - 1));
  let a = Array.create ~len:n 0 in
  let rec dfs i p =
    let f (j, w) =
      a.(j) <- (a.(i) + w) % 2;
      dfs j i
    in
    List.filter g.(i) ~f:(fun (j, _) -> j <> p) |> List.iter ~f
  in
  dfs 0 (-1);
  Array.to_list a
in
answer |> List.map ~f:Int.to_string |> List.iter ~f:Stdlib.print_endline
