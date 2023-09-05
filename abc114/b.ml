open Base;;

let s = Stdlib.read_line () in
let answer =
  List.range 0 (String.length s - 2)
  |> List.map ~f:(fun pos ->
         String.sub s ~pos ~len:3 |> Int.of_string |> ( - ) 753 |> abs)
  |> List.fold ~init:753 ~f:min
in
answer |> Int.to_string |> Stdlib.print_endline
