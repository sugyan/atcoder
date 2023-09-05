open Base;;

let f _ = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun x y -> (x, y)) in
let n, q = f () in
let ab = List.range 0 (n - 1) |> List.map ~f in
let px = List.range 0 q |> List.map ~f in
let answer =
  let g = Array.create ~len:n [] in
  List.iter ab ~f:(fun (a, b) ->
      g.(a - 1) <- (b - 1) :: g.(a - 1);
      g.(b - 1) <- (a - 1) :: g.(b - 1));
  let c = Array.create ~len:n 0 in
  List.iter px ~f:(fun (p, x) -> c.(p - 1) <- c.(p - 1) + x);
  let rec dfs i p =
    let f j =
      c.(j) <- c.(j) + c.(i);
      dfs j i
    in
    List.filter g.(i) ~f:(( <> ) p) |> List.iter ~f
  in
  dfs 0 (-1);
  Array.to_list c
in
answer |> List.map ~f:Int.to_string |> String.concat ~sep:" "
|> Stdlib.print_endline
