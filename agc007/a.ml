open Base;;

let h, w = Caml.Scanf.sscanf (Caml.read_line ()) "%d %d" (fun h w -> (h, w)) in
let a = List.range 0 h |> List.map ~f:(fun _ -> Caml.read_line ()) in
let answer =
  List.sum (module Int) a ~f:(String.count ~f:(Char.( = ) '#')) = h + w - 1
in
answer
|> (function true -> "Possible" | false -> "Impossible")
|> Caml.print_endline
