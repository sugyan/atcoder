open Base;;

let n = Stdlib.read_int () in
let ab =
  List.range 0 n
  |> List.map ~f:(fun _ ->
         Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun a b -> (a, b)))
in
let answer =
  List.sort ab ~compare:(fun (_, b0) (_, b1) -> compare b0 b1)
  |> List.fold_result ~init:0 ~f:(fun acc (a, b) ->
         if acc + a > b then Error () else Ok (acc + a))
  |> Result.is_ok
in
answer |> (function true -> "Yes" | false -> "No") |> Stdlib.print_endline
