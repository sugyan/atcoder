open Base;;

let _ = Stdlib.read_int () in
let s = Stdlib.read_line () in
let answer =
  let c = String.count s ~f:(Char.( = ) '.') in
  let f (b, w) c =
    (if Char.(c = '#') then (b + 1, w) else (b, w - 1)) |> fun (b, w) ->
    ((b, w), b + w)
  in
  String.to_list s
  |> List.folding_map ~init:(0, c) ~f
  |> List.fold ~init:c ~f:min
in
answer |> Int.to_string |> Stdlib.print_endline
