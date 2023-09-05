open Base;;

let read_ints _ =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let _ = Stdlib.read_int () in
let d = read_ints () in
let _ = Stdlib.read_int () in
let t = read_ints () in
let answer =
  Hashtbl.(
    let h = create (module Int) in
    List.iter d ~f:(incr h);
    List.iter t ~f:(decr h);
    for_all h ~f:(( <= ) 0))
in
answer |> (function true -> "YES" | false -> "NO") |> Stdlib.print_endline
