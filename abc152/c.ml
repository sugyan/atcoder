open Base;;

let _ = Stdlib.read_int () in
let p =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  List.folding_map p ~init:(List.hd_exn p) ~f:(fun acc x ->
      (min acc x, x <= acc))
  |> List.count ~f:Fn.id
in
answer |> Int.to_string |> Stdlib.print_endline
