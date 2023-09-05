open Base;;

let read_xy _ =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun x y -> (x, y))
in
let n, m = read_xy () in
let ab = List.range 0 n |> List.map ~f:read_xy in
let cd = List.range 0 m |> List.map ~f:read_xy in
let answer =
  List.map ab ~f:(fun (a, b) ->
      List.mapi cd ~f:(fun i (c, d) -> (abs (a - c) + abs (b - d), i + 1))
      |> List.stable_sort ~compare:Poly.compare
      |> List.hd_exn |> snd)
in
answer |> List.map ~f:Int.to_string |> List.iter ~f:Stdlib.print_endline
