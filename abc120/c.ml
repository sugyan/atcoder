open Base;;

let s = Caml.read_line () in
let answer =
  let f c = String.count s ~f:(Char.equal c) in
  min (f '0') (f '1') * 2
in
answer |> Int.to_string |> Caml.print_endline
