open Base;;

let s = Caml.read_line () in
let answer =
  let rec loop i acc =
    if i = 0 then List.hd_exn acc
    else
      loop (i - 1)
        ((if Char.( = ) s.[i] s.[i - 1] then List.nth_exn acc 2 + 2
         else List.hd_exn acc + 1)
        :: acc)
  in
  loop (String.length s - 1) [ 1; 0; -1 ]
in
answer |> Int.to_string |> Caml.print_endline
