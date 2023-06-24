open Base;;

let s = Caml.read_line () in
let k = Caml.read_int () in
let answer =
  let g = String.to_list s |> List.group ~break:Char.( <> ) in
  let f = List.for_all ~f:(fun l -> List.length l % 2 = 1) in
  let c =
    Char.(s.[0] = (String.rev s).[0]) && f List.[ hd_exn g; last_exn g ]
  in
  if List.length g = 1 then String.length s * k / 2
  else
    k * List.sum (module Int) g ~f:(fun l -> List.length l / 2)
    |> ( + ) (if c then k - 1 else 0)
in
answer |> Int.to_string |> Caml.print_endline
