open Base;;

let n = Caml.read_int () in
let h =
  Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let h = List.to_array h in
  List.range 1 n
  |> List.fold ~init:(0, 0) ~f:(fun (acc, c) i ->
         if h.(i - 1) >= h.(i) then (max acc (c + 1), c + 1) else (acc, 0))
  |> fst
in
answer |> Int.to_string |> Caml.print_endline
