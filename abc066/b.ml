open Base;;

let s = Stdlib.read_line () in
let answer =
  let f len = String.(sub s ~pos:0 ~len = sub s ~pos:len ~len) in
  List.range 0 (String.length s / 2) |> List.rev |> List.find_exn ~f |> ( * ) 2
in
answer |> Int.to_string |> Stdlib.print_endline
