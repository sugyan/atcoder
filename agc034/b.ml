open Base;;

let s = Stdlib.read_line () in
let answer =
  let f t =
    let g (c, s) x = if Char.(x = 'A') then (c + 1, s) else (c, s + c) in
    String.fold t ~init:(0, 0) ~f:g |> snd
  in
  String.substr_replace_all s ~pattern:"BC" ~with_:"E"
  |> String.split_on_chars ~on:[ 'B'; 'C' ]
  |> List.sum (module Int) ~f
in
answer |> Int.to_string |> Stdlib.print_endline
