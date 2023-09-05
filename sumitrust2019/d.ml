open Base;;

let _ = Stdlib.read_int () in
let s = Stdlib.read_line () in
let answer =
  let ok pin =
    let f acc d =
      String.index_from s acc d
      |> Continue_or_stop.(
           function None -> Stop 0 | Some i -> Continue (i + 1))
    in
    String.to_list pin |> List.fold_until ~init:0 ~f ~finish:Fn.id |> ( <> ) 0
  in
  List.range 0 1000 |> List.map ~f:(Printf.sprintf "%03d") |> List.count ~f:ok
in
answer |> Int.to_string |> Stdlib.print_endline
