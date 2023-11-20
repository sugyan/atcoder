open Base;;

let n, k =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun n k -> (n, k))
in
let answer =
  let f b = (n / b * max 0 (b - k)) + max 0 ((n % b) - k + 1) in
  if k = 0 then n * n else List.(init n ~f:Int.succ |> sum (module Int) ~f)
in
answer |> Int.to_string |> Stdlib.print_endline
