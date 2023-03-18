open Base;;

let s = Caml.read_line () in
let answer =
  Char.( = ) s.[0] 'A'
  && Char.is_lowercase s.[1]
  && Char.is_lowercase s.[String.length s - 1]
  && String.count s ~f:(fun c -> Char.( = ) c 'C') = 1
  && String.count s ~f:Char.is_uppercase = 2
in
answer |> (function true -> "AC" | false -> "WA") |> Caml.print_endline
