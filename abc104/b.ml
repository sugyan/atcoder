open Base;;

let s = Stdlib.read_line () in
let answer =
  Char.( = ) s.[0] 'A'
  && Char.is_lowercase s.[1]
  && Char.is_lowercase s.[String.length s - 1]
  && String.count s ~f:(Char.( = ) 'C') = 1
  && String.count s ~f:Char.is_uppercase = 2
in
answer |> (function true -> "AC" | false -> "WA") |> Stdlib.print_endline
