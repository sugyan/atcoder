open Base;;

let _ = Caml.read_int () in
let a =
  Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let b = List.map a ~f:abs |> List.fold ~init:Int.max_value ~f:min in
  List.sum (module Int) a ~f:abs
  - if List.count a ~f:(( > ) 0) % 2 = 0 then 0 else b * 2
in
answer |> Int.to_string |> Caml.print_endline
