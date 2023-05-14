open Base;;

let _ = Caml.read_int () in
let h =
  Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let f i (j, y) x = if x < y then (i, x) else (j, y) in
  let rec loop o = function
    | [] -> 0
    | [ x ] -> x - o
    | l ->
        let i, m = List.foldi l ~init:(0, 100) ~f in
        m - o + loop m (List.take l i) + loop m (List.drop l (i + 1))
  in
  loop 0 h
in
answer |> Int.to_string |> Caml.print_endline
