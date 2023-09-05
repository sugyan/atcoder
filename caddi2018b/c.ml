open Base;;

let n, p = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun n p -> (n, p)) in
let answer =
  let rec f acc m i =
    if i * i > m then m :: acc
    else if m % i = 0 then f (i :: acc) (m / i) i
    else f acc m (i + 1)
  in
  f [] p 2 |> List.group ~break:( <> )
  |> List.map ~f:(fun l -> List.hd_exn l ** (List.length l / n))
  |> List.fold ~init:1 ~f:( * )
in
answer |> Int.to_string |> Stdlib.print_endline
