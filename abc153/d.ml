open Base;;

let h = Caml.read_int () in
let answer =
  let rec loop h = if h = 1 then 1 else 1 + (2 * loop (h / 2)) in
  loop h
in
answer |> Int.to_string |> Caml.print_endline
