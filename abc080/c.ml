open Base;;

let n = Stdlib.read_int () in
let f, p =
  let g _ =
    Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
  in
  let f = List.range 0 n |> List.map ~f:g in
  let p = List.range 0 n |> List.map ~f:g in
  (f, p)
in
let answer =
  let f = List.map f ~f:(List.fold ~init:0 ~f:(fun acc x -> (acc * 2) + x)) in
  let p = List.map p ~f:List.to_array in
  let g x =
    List.zip_exn f p
    |> List.sum (module Int) ~f:(fun (f, p) -> p.(x land f |> Int.popcount))
  in
  List.range 1 1024 |> List.map ~f:g |> List.fold ~init:Int.min_value ~f:max
in
answer |> Int.to_string |> Stdlib.print_endline
