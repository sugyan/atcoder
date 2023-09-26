open Base;;

let x = Stdlib.read_line () in
let answer =
  let f c = function
    | [] -> [ c ]
    | hd :: tl -> if Char.(hd = 'S' && c = 'T') then tl else c :: hd :: tl
  in
  String.fold x ~init:[] ~f:(Fn.flip f) |> List.length
in
answer |> Int.to_string |> Stdlib.print_endline
