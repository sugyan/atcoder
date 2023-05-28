open Base;;

let l, r = Caml.Scanf.sscanf (Caml.read_line ()) "%d %d" (fun l r -> (l, r)) in
let answer =
  let rec c = function
    | [] -> []
    | hd :: tl -> List.map tl ~f:(fun x -> hd * x % 2019) @ c tl
  in
  c (List.range l (min (r + 1) (l + 2019))) |> List.fold ~init:2019 ~f:min
in
answer |> Int.to_string |> Caml.print_endline
