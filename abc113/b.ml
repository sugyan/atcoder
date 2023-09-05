open Base;;

let _ = Stdlib.read_int () in
let t, a = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun t a -> (t, a)) in
let h =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  List.map h ~f:(fun x -> ((a - t) * 1000) + (x * 6) |> abs)
  |> List.foldi ~init:(0, Int.max_value) ~f:(fun i (j, y) x ->
         if x < y then (i + 1, x) else (j, y))
  |> fst
in
answer |> Int.to_string |> Stdlib.print_endline
