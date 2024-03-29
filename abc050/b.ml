open Base;;

let _ = Stdlib.read_int () in
let t =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let m = Stdlib.read_int () in
let px =
  List.range 0 m
  |> List.map ~f:(fun _ ->
         Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun p x -> (p, x)))
in
let answer =
  let sum = List.sum (module Int) t ~f:Fn.id in
  let t = List.to_array t in
  List.map px ~f:(fun (p, x) -> sum - t.(p - 1) + x)
in
answer |> List.map ~f:Int.to_string |> List.iter ~f:Stdlib.print_endline
