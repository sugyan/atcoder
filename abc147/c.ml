open Base;;

let n = Stdlib.read_int () in
let f _ =
  let a = Stdlib.read_int () in
  let g _ =
    Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun x y -> (x, y))
  in
  List.range 0 a |> List.map ~f:g
in
let xy = List.range 0 n |> List.map ~f in
let answer =
  let f i =
    let rec g = function
      | [] -> true
      | (x, b) :: tl ->
          if Bool.( = ) (i land (1 lsl (x - 1)) <> 0) (b = 1) then g tl
          else false
    in
    List.filteri xy ~f:(fun j _ -> i land (1 lsl j) <> 0) |> List.concat |> g
    |> function
    | true -> Some (Int.popcount i)
    | false -> None
  in
  List.range 0 (1 lsl n) |> List.filter_map ~f |> List.fold ~init:0 ~f:max
in
answer |> Int.to_string |> Stdlib.print_endline
