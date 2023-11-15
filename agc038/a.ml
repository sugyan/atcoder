open Base;;

let h, w, a, b =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d %d %d" (fun h w a b ->
      (h, w, a, b))
in
let answer =
  let f i =
    let g j = if Bool.( = ) (i < b) (j < a) then '1' else '0' in
    List.range 0 w |> List.map ~f:g |> String.of_char_list
  in
  List.range 0 h |> List.map ~f
in
answer |> List.iter ~f:Stdlib.print_endline
