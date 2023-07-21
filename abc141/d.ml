open Base;;

let m = Caml.Scanf.sscanf (Caml.read_line ()) "%d %d" (fun _ m -> m) in
let a =
  Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let f v = Option.value v ~default:0 + 1 in
  let g acc =
    let k, v = Map.max_elt_exn acc in
    (if v = 1 then Map.remove acc k else Map.set acc ~key:k ~data:(v - 1))
    |> Fn.flip Map.update (k / 2) ~f
  in
  List.fold a ~init:(Map.empty (module Int)) ~f:(Map.update ~f)
  |> Fn.apply_n_times ~n:m g |> Map.to_alist
  |> List.sum (module Int) ~f:(fun (k, v) -> k * v)
in
answer |> Int.to_string |> Caml.print_endline
