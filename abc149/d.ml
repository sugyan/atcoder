open Base;;

let _, k = Caml.Scanf.sscanf (Caml.read_line ()) "%d %d" (fun n k -> (n, k)) in
let r, s, p =
  Caml.Scanf.sscanf (Caml.read_line ()) "%d %d %d" (fun r s p -> (r, s, p))
in
let t = Caml.read_line () in
let answer =
  let a = String.to_array t in
  let f i c =
    if i < k || Char.(a.(i - k) <> c) then
      match c with 'r' -> p | 's' -> r | 'p' -> s | _ -> 0
    else (
      a.(i) <- ' ';
      0)
  in
  String.to_list t |> List.mapi ~f |> List.sum (module Int) ~f:Fn.id
in
answer |> Int.to_string |> Caml.print_endline
