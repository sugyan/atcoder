open Base;;

let n = Stdlib.read_int () in
let ab =
  let f _ =
    Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun a b -> (a, b))
  in
  List.range 0 (n - 1) |> List.map ~f
in
let answer =
  let g, a = Array.(create ~len:n [], create ~len:(n - 1) 0) in
  List.iteri ab ~f:(fun i (a, b) ->
      g.(a - 1) <- (b - 1, i) :: g.(a - 1);
      g.(b - 1) <- (a - 1, i) :: g.(b - 1));
  let rec dfs i (p, c) =
    let f acc (j, k) =
      let acc = if acc = c then acc + 1 else acc in
      a.(k) <- acc;
      dfs j (i, acc);
      acc + 1
    in
    g.(i)
    |> List.filter ~f:(fun (j, _) -> j <> p)
    |> List.fold ~init:1 ~f |> ignore
  in
  dfs 0 (-1, 0);
  Array.fold a ~init:0 ~f:max :: Array.to_list a
in
answer |> List.map ~f:Int.to_string |> List.iter ~f:Stdlib.print_endline
