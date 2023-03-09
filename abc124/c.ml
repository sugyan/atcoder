open Base;;

let s = Caml.read_line () in
let answer =
  String.to_list s
  |> List.counti ~f:(fun i -> Char.( = ) (if i % 2 = 0 then '0' else '1'))
  |> fun x -> min x (String.length s - x)
in
answer |> Int.to_string |> Caml.print_endline
