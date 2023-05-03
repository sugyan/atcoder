open Base;;

let _ = Caml.read_int () in
let a =
  Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let rec f = function
    | x :: y :: z :: xs ->
        if Bool.( <> ) (x > y) (y > z) then 1 + f (z :: xs) else f (y :: z :: xs)
    | _ -> 1
  in
  List.remove_consecutive_duplicates a ~equal |> f
in
answer |> Int.to_string |> Caml.print_endline
