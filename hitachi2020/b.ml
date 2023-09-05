open Base;;

let m = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d %d" (fun _ _ m -> m) in
let a =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let b =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let cs =
  List.init m ~f:(fun _ ->
      Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d %d" (fun x y c -> (x, y, c)))
in
let answer =
  let a = Array.of_list a in
  let b = Array.of_list b in
  let init =
    Array.fold a ~init:a.(0) ~f:min + Array.fold b ~init:b.(0) ~f:min
  in
  List.fold cs ~init ~f:(fun acc (x, y, c) ->
      min acc (a.(x - 1) + b.(y - 1) - c))
in
answer |> Int.to_string |> Stdlib.print_endline
