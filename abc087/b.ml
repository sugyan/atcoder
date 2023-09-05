open Base;;

let a = Stdlib.read_int () in
let b = Stdlib.read_int () in
let c = Stdlib.read_int () in
let x = Stdlib.read_int () in
let answer =
  let rec loop x = function
    | [] -> if x = 0 then 1 else 0
    | (v, n) :: tl ->
        List.init (n + 1) ~f:(( * ) v)
        |> List.sum (module Int) ~f:(fun y -> loop (x - y) tl)
  in
  loop x [ (500, a); (100, b); (50, c) ]
in
answer |> Int.to_string |> Stdlib.print_endline
