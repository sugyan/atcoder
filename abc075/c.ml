open Base;;

let f _ =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun x y -> (x, y))
in
let n, m = f () in
let ab = List.range 0 m |> List.map ~f in
let answer =
  let g = Array.create ~len:n [] in
  List.iter ab ~f:(fun (a, b) ->
      g.(a - 1) <- (b - 1) :: g.(a - 1);
      g.(b - 1) <- (a - 1) :: g.(b - 1));
  let ord, low = Array.(create ~len:n (-1), create ~len:n (-1)) in
  let k = ref 0 in
  let rec dfs u p =
    Int.incr k;
    ord.(u) <- !k;
    low.(u) <- !k;
    let f v =
      if ord.(v) = -1 then (
        dfs v u;
        low.(u) <- min low.(u) low.(v))
      else if v <> p then low.(u) <- min low.(u) ord.(v)
    in
    List.iter g.(u) ~f
  in
  dfs 0 (-1);
  List.count ab ~f:(fun (a, b) ->
      ord.(a - 1) < low.(b - 1) || ord.(b - 1) < low.(a - 1))
in
answer |> Int.to_string |> Stdlib.print_endline
