open Base;;

let x, y = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun x y -> (x, y)) in
let answer =
  List.filter_map
    [ (x, y, 0); (-x, y, 1); (x, -y, 1); (-x, -y, 2) ]
    ~f:(fun (a, b, i) -> if a > b then None else Some (b - a + i))
  |> List.fold ~init:Int.max_value ~f:min
in
answer |> Int.to_string |> Stdlib.print_endline
