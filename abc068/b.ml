open Base;;

let n = Stdlib.read_int () in
List.range 0 7 |> List.rev
|> List.map ~f:(( lsl ) 1)
|> List.find_exn ~f:(Fn.flip ( <= ) n)
|> Int.to_string |> Stdio.print_endline
