open Base;;

let x = Caml.Scanf.sscanf (Caml.read_line ()) "%d %d %d" (fun _ _ x -> x) in
let a =
  Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer = min (List.count a ~f:(( < ) x)) (List.count a ~f:(( > ) x)) in
answer |> Int.to_string |> Stdio.print_endline
