open Base;;

let n = Stdlib.read_int () in
let d, x = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun d x -> (d, x)) in
let a = List.init n ~f:(fun _ -> Stdlib.read_int ()) in
let answer =
  List.map a ~f:(fun a -> ((d - 1) / a) + 1) |> List.fold ~init:x ~f:( + )
in
answer |> Int.to_string |> Stdlib.print_endline
