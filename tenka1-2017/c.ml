open Base;;

let n = Stdlib.read_int () in
let answer =
  let f x =
    let g y =
      let num, den = (n * x * y, (4 * x * y) - (n * (x + y))) in
      if den > 0 && num % den = 0 then Some (x, y, num / den) else None
    in
    List.range x 3501 |> List.find_map ~f:g
  in
  List.range 1 3501 |> List.find_map_exn ~f
in
answer |> fun (x, y, z) ->
Printf.sprintf "%d %d %d" x y z |> Stdlib.print_endline
