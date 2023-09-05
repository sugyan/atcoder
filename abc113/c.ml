open Base;;

let f _ = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun a b -> (a, b)) in
let n, m = f () in
let py = List.range 0 m |> List.map ~f:(fun _ -> f ()) in
let answer =
  let a = Array.create ~len:n (Map.empty (module Int)) in
  List.iteri py ~f:(fun i (p, y) ->
      a.(p - 1) <- Map.add_exn a.(p - 1) ~key:y ~data:i);
  let b = Array.create ~len:m "" in
  let f i m =
    let g k j = b.(j) <- Printf.sprintf "%06d%06d" (i + 1) (k + 1) in
    Map.data m |> List.iteri ~f:g
  in
  Array.iteri a ~f;
  Array.to_list b
in
answer |> List.iter ~f:Stdlib.print_endline
