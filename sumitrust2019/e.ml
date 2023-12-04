open Base;;

let _ = Stdlib.read_int () in
let a =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let c = Array.create ~len:3 0 in
  let f acc x =
    let idx = List.(range 0 3 |> filter ~f:(fun i -> c.(i) = x)) in
    List.hd idx |> Option.iter ~f:(fun i -> c.(i) <- c.(i) + 1);
    acc * List.length idx % 1_000_000_007
  in
  List.fold a ~init:1 ~f
in
answer |> Int.to_string |> Stdlib.print_endline
