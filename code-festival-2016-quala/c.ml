open Base;;

let s = Stdlib.read_line () in
let k = Stdlib.read_int () in
let answer =
  let a = Char.to_int 'a' in
  let f i acc c =
    let d = (a + 26 - c) % 26 in
    if i = String.length s - 1 then (0, a + ((c - a + acc) % 26))
    else if d <= acc then (acc - d, a)
    else (acc, c)
  in
  String.to_list s |> List.map ~f:Char.to_int
  |> List.folding_mapi ~init:k ~f
  |> List.map ~f:Char.of_int_exn
  |> String.of_char_list
in
answer |> Stdlib.print_endline
