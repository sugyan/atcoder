open Base;;

let h, w = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun h w -> (h, w)) in
let s = List.range 0 h |> List.map ~f:(fun _ -> Stdlib.read_line ()) in
let answer =
  let s = List.to_array s in
  let f (i, j) = i >= 0 && h > i && j >= 0 && w > j && Char.(s.(i).[j] = '#') in
  let is_ok (i, j) =
    Char.(s.(i).[j] = '.')
    || List.exists [ (i - 1, j); (i, j - 1); (i + 1, j); (i, j + 1) ] ~f
  in
  List.cartesian_product (List.range 0 h) (List.range 0 w)
  |> List.for_all ~f:is_ok
in
answer |> (function true -> "Yes" | false -> "No") |> Stdlib.print_endline
