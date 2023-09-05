open Base;;

let a, b, c, x, y =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d %d %d %d" (fun a b c x y ->
      (a, b, c, x, y))
in
let answer =
  List.range 0 (max x y + 1)
  |> List.map ~f:(fun i ->
         (c * 2 * i) + (a * max 0 (x - i)) + (b * max 0 (y - i)))
  |> List.fold ~init:Int.max_value ~f:min
in
answer |> Int.to_string |> Stdlib.print_endline
