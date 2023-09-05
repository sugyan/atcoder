open Base;;

let n = Stdlib.read_int () in
let s = List.range 0 n |> List.map ~f:(fun _ -> Stdlib.read_line ()) in
let answer =
  let a = [| 0; 0; 0 |] in
  List.iter s ~f:(fun s ->
      match (s.[0], s.[String.length s - 1]) with
      | 'B', 'A' -> a.(0) <- a.(0) + 1
      | 'B', _ -> a.(1) <- a.(1) + 1
      | _, 'A' -> a.(2) <- a.(2) + 1
      | _ -> ());
  let f s =
    List.range 1 (String.length s)
    |> List.count ~f:(fun i -> Char.(s.[i - 1] = 'A' && s.[i] = 'B'))
  in
  let m = min a.(1) a.(2) in
  (if a.(0) = 0 then m else if a.(1) + a.(2) = 0 then a.(0) - 1 else a.(0) + m)
  |> ( + ) (List.sum (module Int) s ~f)
in
answer |> Int.to_string |> Stdlib.print_endline
