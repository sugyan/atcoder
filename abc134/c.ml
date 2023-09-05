open Base;;

let n = Stdlib.read_int () in
let a = List.range 0 n |> List.map ~f:(fun _ -> Stdlib.read_int ()) in
let answer =
  let s = List.sort a ~compare:descending |> List.to_array in
  List.map a ~f:(fun x -> s.(x = s.(0) |> Bool.to_int))
in
answer |> List.map ~f:Int.to_string |> List.iter ~f:Stdlib.print_endline
