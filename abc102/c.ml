open Base;;

let n = Stdlib.read_int () in
let a =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let a = List.mapi a ~f:(fun i x -> x - i) |> List.sort ~compare in
  List.nth_exn a (n / 2) |> fun m ->
  List.(map a ~f:(( - ) m) |> sum (module Int) ~f:abs)
in
answer |> Int.to_string |> Stdlib.print_endline
