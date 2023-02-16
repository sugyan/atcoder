open Base;;

Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
|> (function [ a; b ] -> (a + b - 3) / (a - 1) | _ -> assert false)
|> Int.to_string |> Caml.print_endline
