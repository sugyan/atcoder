open Base;;

let n = Caml.read_int () in
let s = Caml.read_line () in
let answer =
  let s = String.to_list s in
  let f c acc x = if Char.(x = c) then (acc + 1, acc + 1) else (acc, acc) in
  let lw = List.folding_map s ~init:0 ~f:(f 'W') in
  let le = List.folding_map (List.rev s) ~init:0 ~f:(f 'E') |> List.rev in
  List.map2_exn lw le ~f:(fun w e -> w + e - 1) |> List.fold ~init:n ~f:min
in
answer |> Int.to_string |> Caml.print_endline
