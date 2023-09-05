open Base;;

let n =
  Stdlib.read_line () |> String.split ~on:' ' |> String.concat |> Int.of_string
in
List.range 1 n
|> List.map ~f:(Fn.flip ( ** ) 2)
|> List.exists ~f:(( = ) n)
|> (function true -> "Yes" | false -> "No")
|> Stdio.print_endline
