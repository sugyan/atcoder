open Base;;

let n = Stdlib.read_int () in
let s = Stdlib.read_line () in
let answer =
  let f acc x = (acc + if Char.(x = '(') then -1 else 1) |> fun x -> (x, x) in
  let c =
    String.to_list s |> List.folding_map ~init:0 ~f |> List.fold ~init:0 ~f:max
  in
  let d = String.count s ~f:(Char.( = ) '(') in
  String.make c '(' ^ s ^ String.make (c + d + d - n) ')'
in
answer |> Stdlib.print_endline
