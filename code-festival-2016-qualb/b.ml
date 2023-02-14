open Core;;

let solve _ a b s =
  let f (ab, bb) = function
    | 'a' when ab < a + b -> ((ab + 1, bb), true)
    | 'b' when ab < a + b && bb < b -> ((ab + 1, bb + 1), true)
    | _ -> ((ab, bb), false)
  in
  String.to_list s
  |> List.folding_map ~init:(0, 0) ~f
  |> List.map ~f:(function true -> "Yes" | false -> "No")
  |> List.iter ~f:print_endline
in
Scanf.scanf "%d %d %d %s" solve
