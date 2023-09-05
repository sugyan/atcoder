open Base;;

let k, n =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
  |> function
  | [ k; n ] -> (k, n)
  | _ -> assert false
in
let a =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let a = a @ [ List.hd_exn a + k ] |> List.to_array in
  let f acc i = a.(i + 1) - a.(i) |> max acc in
  List.range 0 n |> List.fold ~init:0 ~f |> ( - ) k
in
Int.to_string answer |> Stdlib.print_endline
