open Base;;

let s = Stdlib.read_int () in
let answer =
  let f n = if n % 2 = 0 then n / 2 else (3 * n) + 1 in
  let rec loop acc =
    let a = List.hd_exn acc |> f in
    if List.mem acc a ~equal then List.length acc + 1 else loop (a :: acc)
  in
  loop [ s ]
in
answer |> Int.to_string |> Stdio.print_endline
