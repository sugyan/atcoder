open Base;;

let n, m = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun n m -> (n, m)) in
let a = List.range 0 n |> List.map ~f:(fun _ -> Stdlib.read_line ()) in
let b = List.range 0 m |> List.map ~f:(fun _ -> Stdlib.read_line ()) in
let answer =
  let a, b = (List.to_array a, List.to_array b) in
  let f i =
    let g j =
      List.cartesian_product (List.range 0 m) (List.range 0 m)
      |> List.for_all ~f:(fun (k, l) -> Char.(a.(i + k).[j + l] = b.(k).[l]))
    in
    List.range 0 (n - m + 1) |> List.exists ~f:g
  in
  List.range 0 (n - m + 1) |> List.exists ~f
in
answer |> (function true -> "Yes" | false -> "No") |> Stdlib.print_endline
