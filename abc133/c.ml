open Base;;

let l, r = Caml.Scanf.sscanf (Caml.read_line ()) "%d %d" (fun l r -> (l, r)) in
let answer =
  if l / 2019 = r / 2019 then
    let rec c = function
      | [] -> []
      | hd :: tl -> List.map tl ~f:(fun x -> hd * x % 2019) @ c tl
    in
    c (List.range l (r + 1)) |> List.fold ~init:2019 ~f:min
  else 0
in
answer |> Int.to_string |> Caml.print_endline
