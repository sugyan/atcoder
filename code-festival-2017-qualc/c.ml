open Base;;

let s = Stdlib.read_line () in
let answer =
  let ss = String.filter s ~f:(Char.( <> ) 'x') in
  if String.(ss = rev ss) then
    String.fold s ~init:(0, []) ~f:(fun (cnt, lst) c ->
        if Char.(c = 'x') then (cnt + 1, lst) else (0, cnt :: lst))
    |> (fun (cnt, lst) -> cnt :: lst)
    |> (fun l -> List.(zip_exn l (rev l)))
    |> List.sum (module Int) ~f:(fun (a, b) -> a - b |> abs)
    |> Fn.flip ( / ) 2
  else -1
in
answer |> Int.to_string |> Stdlib.print_endline
