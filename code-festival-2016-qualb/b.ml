open Base;;

let nab = Caml.read_line () in
let s = Caml.read_line () in
let solve (a, b) =
  let f (ab, bb) = function
    | 'a' when ab < a + b -> ((ab + 1, bb), true)
    | 'b' when ab < a + b && bb < b -> ((ab + 1, bb + 1), true)
    | _ -> ((ab, bb), false)
  in
  String.to_list s
  |> List.folding_map ~init:(0, 0) ~f
  |> List.map ~f:(function true -> "Yes" | false -> "No")
in
nab |> String.split ~on:' ' |> List.map ~f:Int.of_string
|> (function [ _; a; b ] -> solve (a, b) | _ -> assert false)
|> List.iter ~f:Caml.print_endline
