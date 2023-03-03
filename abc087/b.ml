open Base;;

let a = Caml.read_int () in
let b = Caml.read_int () in
let c = Caml.read_int () in
let x = Caml.read_int () in
let answer =
  let rec loop x = function
    | [] -> if x = 0 then 1 else 0
    | (v, n) :: tl ->
        List.init (n + 1) ~f:(( * ) v)
        |> List.sum (module Int) ~f:(fun y -> loop (x - y) tl)
  in
  loop x [ (500, a); (100, b); (50, c) ]
in
answer |> Int.to_string |> Caml.print_endline
