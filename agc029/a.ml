open Base;;

let s = Caml.read_line () in
let answer =
  String.fold s ~init:(0, 0) ~f:(fun (a, b) c ->
      if Char.( = ) 'B' c then (a, b + 1) else (a + b, b))
  |> fst
in
answer |> Int.to_string |> Caml.print_endline
