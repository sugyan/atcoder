open Base;;

let n, m = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun n m -> (n, m)) in
let x =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let a = List.sort x ~compare |> List.to_array in
  List.init (m - 1) ~f:(fun i -> a.(i + 1) - a.(i))
  |> List.sort ~compare
  |> Fn.flip List.take (m - n)
  |> List.fold ~init:0 ~f:( + )
in
answer |> Int.to_string |> Stdlib.print_endline
