open Base;;

let _ = Stdlib.read_int () in
let a =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  List.fold a
    ~init:(Map.empty (module Int))
    ~f:(Map.update ~f:(function None -> 1 | Some x -> x + 1))
  |> Map.to_alist
  |> List.sort ~compare:Poly.compare
  |> function
  | [ (0, _) ] -> true
  | [ (0, c0); (_, c1) ] -> c0 * 2 = c1
  | [ (a0, c0); (a1, c1); (a2, c2) ] -> a0 lxor a1 = a2 && c0 = c1 && c1 = c2
  | _ -> false
in
answer |> (function true -> "Yes" | false -> "No") |> Stdlib.print_endline
