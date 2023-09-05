open Base;;

let n = Stdlib.read_float () in
let x =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let m =
  List.sum (module Int) x ~f:Fn.id
  |> Float.of_int |> Fn.flip ( /. ) n |> Float.round_nearest |> Int.of_float
in
x
|> List.map ~f:(( - ) m)
|> List.map ~f:(Fn.flip Int.( ** ) 2)
|> List.sum (module Int) ~f:Fn.id
|> Int.to_string |> Stdlib.print_endline
