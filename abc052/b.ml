open Base;;

let _ = Stdlib.read_int () in
let s = Stdlib.read_line () in
let answer =
  String.fold s ~init:(0, 0) ~f:(fun (acc, m) -> function
    | 'I' -> (acc + 1, max m (acc + 1))
    | 'D' -> (acc - 1, max m (acc - 1))
    | _ -> assert false)
  |> snd
in
answer |> Int.to_string |> Stdlib.print_endline
