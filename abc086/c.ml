open Base;;

let n = Caml.read_int () in
let txy =
  let f _ =
    Caml.Scanf.sscanf (Caml.read_line ()) "%d %d %d" (fun t x y -> (t, x, y))
  in
  List.range 0 n |> List.map ~f
in
let answer =
  List.fold_result txy ~init:(0, 0, 0) ~f:(fun (t0, x0, y0) (t1, x1, y1) ->
      let d = t1 - t0 - (abs (x1 - x0) + abs (y1 - y0)) in
      if d >= 0 && d % 2 = 0 then Ok (t1, x1, y1) else Error ())
  |> Result.is_ok
in
answer |> (function true -> "Yes" | false -> "No") |> Caml.print_endline
