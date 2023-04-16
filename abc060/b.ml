open Base;;

let a, b, c =
  Caml.Scanf.sscanf (Caml.read_line ()) "%d %d %d" (fun a b c -> (a, b, c))
in
let answer = List.init b ~f:(fun i -> i * a % b) |> List.exists ~f:(( = ) c) in
answer |> (function true -> "YES" | false -> "NO") |> Caml.print_endline
