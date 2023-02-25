open Base;;

let n = Caml.read_int () in
let d, x = Caml.Scanf.sscanf (Caml.read_line ()) "%d %d" (fun d x -> (d, x)) in
let a = List.init n ~f:(fun _ -> Caml.read_int ()) in
let answer =
  List.map a ~f:(fun a -> ((d - 1) / a) + 1) |> List.fold ~init:x ~f:( + )
in
answer |> Int.to_string |> Caml.print_endline
