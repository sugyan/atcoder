open Base;;

let n = Caml.read_int () in
let f _ =
  Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let a = f () in
let b = f () in
let answer =
  List.zip_exn a b
  |> List.map ~f:(fun (a, b) -> a - b)
  |> List.sort ~compare
  |> List.split_while ~f:(fun x -> x < 0)
  |> fun (nega, posi) ->
  List.rev posi
  |> List.fold_until
       ~init:(List.fold nega ~init:0 ~f:( + ), 0)
       ~f:(fun (sum, c) x ->
         if sum < 0 then Continue (sum + x, c + 1)
         else Stop (c + List.length nega))
       ~finish:(fun (sum, _) -> if sum < 0 then -1 else n)
in
answer |> Int.to_string |> Caml.print_endline
