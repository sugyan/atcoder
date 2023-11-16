open Base;;

let s = Stdlib.read_line () in
let k = Stdlib.read_int () in
let answer =
  let f len =
    let g pos = String.sub s ~pos ~len in
    List.range 0 (String.length s - len + 1) |> List.map ~f:g
  in
  List.range 1 (k + 1)
  |> List.concat_map ~f
  |> List.dedup_and_sort ~compare:String.compare
  |> Fn.flip List.nth_exn (k - 1)
in
answer |> Stdlib.print_endline
