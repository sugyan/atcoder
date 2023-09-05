open Base;;

let n, m = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun n m -> (n, m)) in
let answer =
  let init = n - m |> abs |> function 0 -> 2 | 1 -> 1 | _ -> 0 in
  let f x y = x * y % 1_000_000_007 in
  List.concat [ List.range 1 (n + 1); List.range 1 (m + 1) ]
  |> List.fold ~init ~f
in
answer |> Int.to_string |> Stdlib.print_endline
