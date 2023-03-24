open Base;;

let s = Caml.read_line () in
let answer =
  let rec loop i (p0, p1, p2) =
    if i = String.length s then p0
    else
      loop (i + 1)
        (if Char.( = ) s.[i] s.[i - 1] then (p2 + 2, p0, p1)
        else (p0 + 1, p0, p1))
  in
  loop 1 (1, 0, -1)
in
answer |> Int.to_string |> Caml.print_endline
