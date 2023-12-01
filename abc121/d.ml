open Base;;

let a, b =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun a b -> (a, b))
in
let answer =
  let f n = (n + 1) / 2 % 2 lxor if n % 2 = 0 then n else 0 in
  f b lxor f (a - 1)
in
answer |> Int.to_string |> Stdlib.print_endline
