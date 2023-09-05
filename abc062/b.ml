open Base;;

let h, w = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun h w -> (h, w)) in
let a = List.range 0 h |> List.map ~f:(fun _ -> Stdlib.read_line ()) in
let answer =
  let s = String.make (w + 2) '#' in
  List.concat [ [ s ]; List.map a ~f:(Printf.sprintf "#%s#"); [ s ] ]
in
answer |> List.iter ~f:(fun s -> Stdlib.print_endline s)
