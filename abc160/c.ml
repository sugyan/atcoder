open Base;;

let k, n =
  Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
  |> function
  | [ k; n ] -> (k, n)
  | _ -> assert false
in
let a =
  Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let a = List.to_array a in
  let f i acc x = (a.((i + 1) % n) - x) % k |> Int.max acc in
  Array.foldi a ~init:0 ~f |> ( - ) k
in
Int.to_string answer |> Caml.print_endline
