open Core;;

let solve h w = if h = 1 || w = 1 then 1 else ((h * w) + 1) / 2 in
Scanf.scanf "%d %d" solve |> Int.to_string |> print_endline
