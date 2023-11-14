open Base;;

let s = Stdlib.read_line () in
let answer =
  let rec f sum n = function
    | [] -> sum + n
    | x :: xs -> f (sum + n) x xs + f sum ((n * 10) + x) xs
  in
  String.to_list s |> List.map ~f:(fun c -> Char.(to_int c - to_int '0'))
  |> function
  | hd :: tl -> f 0 hd tl
  | _ -> 0
in
answer |> Int.to_string |> Stdlib.print_endline
