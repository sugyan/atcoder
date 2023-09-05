open Base;;

let n, m, k =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d %d" (fun n m k -> (n, m, k))
in
let answer =
  List.(
    cartesian_product (range 0 (n + 1)) (range 0 (m + 1))
    |> exists ~f:(fun (i, j) -> (i * m) + (j * n) - (2 * i * j) = k))
in
answer |> (function true -> "Yes" | false -> "No") |> Stdlib.print_endline
