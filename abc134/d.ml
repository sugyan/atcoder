open Base;;

let n = Caml.read_int () in
let a =
  Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let b = Array.create ~len:n 0 in
  let rec f i j = if j >= n then 0 else b.(j) + f i (j + i + 1) in
  List.(
    mapi a ~f:(fun i x -> (i, x))
    |> rev
    |> iter ~f:(fun (i, x) -> b.(i) <- (f i i + x) % 2);
    range 0 n |> filter ~f:(fun i -> b.(i) = 1) |> map ~f:(( + ) 1))
in
List.length answer |> Int.to_string |> Caml.print_endline;
List.map answer ~f:Int.to_string |> String.concat ~sep:" " |> Caml.print_endline
