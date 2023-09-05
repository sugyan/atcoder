open Base;;

let _ = Stdlib.read_int () in
let h =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  List.fold_result h ~init:0 ~f:(fun acc x ->
      if x < acc then Error () else Ok (max acc (x - 1)))
  |> Result.is_ok
in
answer |> (function true -> "Yes" | false -> "No") |> Stdlib.print_endline
