open Base;;

let _ = Stdlib.read_int () in
let a =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  List.(
    sort a ~compare:descending |> group ~break:( <> )
    |> filter ~f:(fun l -> length l >= 2)
    |> function
    | l :: _ when length l >= 4 -> hd_exn l * hd_exn l
    | l0 :: l1 :: _ -> hd_exn l0 * hd_exn l1
    | _ -> 0)
in
answer |> Int.to_string |> Stdlib.print_endline
