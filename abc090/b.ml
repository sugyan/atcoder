open Base;;

let a, b = Caml.Scanf.sscanf (Caml.read_line ()) "%d %d" (fun a b -> (a, b)) in
let answer =
  List.range a (b + 1)
  |> List.map ~f:Int.to_string
  |> List.count ~f:(fun s -> Poly.(s = String.rev s))
in
answer |> Int.to_string |> Caml.print_endline
