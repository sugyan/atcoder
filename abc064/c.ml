open Base;;

let n = Caml.read_int () in
let a =
  Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let b = List.map a ~f:(Fn.flip ( / ) 400) |> List.filter ~f:(( > ) 8) in
  let c = List.dedup_and_sort b ~compare |> List.length in
  (max c 1, c + n - List.length b)
in
answer
|> (fun (min, max) -> Printf.sprintf "%d %d" min max)
|> Caml.print_endline
