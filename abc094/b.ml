open Base;;

let x = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d %d" (fun _ _ x -> x) in
let a =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer = min (List.count a ~f:(( < ) x)) (List.count a ~f:(( > ) x)) in
answer |> Int.to_string |> Stdio.print_endline
