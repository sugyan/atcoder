open Base;;

let n = Caml.read_int () in
let csf =
  let f _ =
    Caml.Scanf.sscanf (Caml.read_line ()) "%d %d %d" (fun c s f -> (c, s, f))
  in
  List.range 0 (n - 1) |> List.map ~f
in
let answer =
  let g =
    List.fold ~init:0 ~f:(fun acc (c, s, f) ->
        acc + c + (s - acc |> fun x -> if x > 0 then x else x % f))
  in
  List.init n ~f:(fun i -> List.drop csf i |> g)
in
answer |> List.iter ~f:(fun a -> Int.to_string a |> Caml.print_endline)
