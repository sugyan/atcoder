open Base;;

let _ = Caml.read_int () in
let p =
  Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  List.mapi p ~f:(fun i x -> i + 1 = x)
  |> List.fold ~init:(0, true) ~f:(fun (acc, p) x ->
         if p && x then (acc + 1, false) else (acc, true))
  |> fst
in
answer |> Int.to_string |> Caml.print_endline
