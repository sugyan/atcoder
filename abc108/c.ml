open Base;;

let n, k =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun n k -> (n, k))
in
let answer =
  ((n / k) ** 3) + if k % 2 = 0 then ((n + (k / 2)) / k) ** 3 else 0
in
answer |> Int.to_string |> Stdlib.print_endline
