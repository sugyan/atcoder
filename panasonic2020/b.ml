open Base;;

let solve (h, w) = if h = 1 || w = 1 then 1 else ((h * w) + 1) / 2 in
Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
|> (function [ h; w ] -> solve (h, w) | _ -> assert false)
|> Int.to_string |> Caml.print_endline
