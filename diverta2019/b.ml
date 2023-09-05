open Base;;

let rgb, n =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d %d %d" (fun r g b n ->
      ([ r; g; b ], n))
in
let answer =
  let rec f n = function
    | [] -> 0
    | [ e ] -> if n % e = 0 then 1 else 0
    | hd :: tl ->
        List.init ((n / hd) + 1) ~f:(fun i -> f (n - (hd * i)) tl)
        |> List.sum (module Int) ~f:Fn.id
  in
  f n rgb
in
answer |> Int.to_string |> Stdlib.print_endline
