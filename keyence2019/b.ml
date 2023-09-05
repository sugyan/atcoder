open Base;;

let s = Stdlib.read_line () in
let answer =
  let len = String.length s in
  List.init 8 ~f:(fun i ->
      String.sub s ~pos:0 ~len:i ^ String.sub s ~pos:(i + len - 7) ~len:(7 - i))
  |> Fn.flip List.mem "keyence" ~equal:String.equal
in
answer |> (function true -> "YES" | false -> "NO") |> Stdlib.print_endline
