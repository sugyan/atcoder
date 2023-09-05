open Base;;

let s = Stdlib.read_line () in
let answer =
  let f c = String.count s ~f:(Char.equal c) in
  min (f '0') (f '1') * 2
in
answer |> Int.to_string |> Stdlib.print_endline
