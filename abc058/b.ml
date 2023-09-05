open Base;;

let o = Stdlib.read_line () in
let e = Stdlib.read_line () in
let answer =
  String.init
    (String.length (o ^ e))
    ~f:(fun i -> if i % 2 = 0 then o.[i / 2] else e.[i / 2])
in
answer |> Stdlib.print_endline
