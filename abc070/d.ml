open Base;;

let n = Stdlib.read_int () in
let abc =
  let f _ =
    Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d %d" (fun a b c ->
        (a, b, c))
  in
  List.range 0 (n - 1) |> List.map ~f
in
let k, xy =
  let f _ =
    Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun x y -> (x, y))
  in
  let q, k = f () in
  let xy = List.range 0 q |> List.map ~f in
  (k, xy)
in
let answer =
  let g = Array.create ~len:n [] in
  List.iter abc ~f:(fun (a, b, c) ->
      g.(a - 1) <- (b - 1, c) :: g.(a - 1);
      g.(b - 1) <- (a - 1, c) :: g.(b - 1));
  let a = Array.create ~len:n 0 in
  let rec dfs g i d p =
    a.(i) <- d;
    g.(i) |> List.iter ~f:(fun (j, c) -> if j <> p then dfs g j (d + c) i)
  in
  dfs g (k - 1) 0 (-1);
  List.map xy ~f:(fun (x, y) -> a.(x - 1) + a.(y - 1))
in
answer |> List.map ~f:Int.to_string |> List.iter ~f:Stdlib.print_endline
