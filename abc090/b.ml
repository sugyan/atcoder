open Base;;

let a, b = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun a b -> (a, b)) in
let answer =
  List.range a (b + 1)
  |> List.map ~f:Int.to_string
  |> List.count ~f:(fun s -> Poly.(s = String.rev s))
in
answer |> Int.to_string |> Stdlib.print_endline
