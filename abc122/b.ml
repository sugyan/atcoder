open Base;;

let s = Stdlib.read_line () in
let answer =
  String.to_list s
  |> List.fold ~init:(0, 0) ~f:(fun (c, m) -> function
       | 'A' | 'C' | 'G' | 'T' -> (c + 1, max m (c + 1)) | _ -> (0, m))
  |> snd
in
answer |> Int.to_string |> Stdlib.print_endline
