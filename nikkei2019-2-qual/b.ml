open Base;;

let n = Caml.read_int () in
let d =
  Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let a = Array.create ~len:n 0 in
  List.iter d ~f:(fun x -> a.(x) <- a.(x) + 1);
  let f acc i =
    Fn.apply_n_times ~n:a.(i + 1) (fun x -> x * a.(i) % 998_244_353) acc
  in
  if List.hd_exn d = 0 && a.(0) = 1 then
    List.range 0 (List.fold d ~init:0 ~f:max) |> List.fold ~init:1 ~f
  else 0
in
answer |> Int.to_string |> Caml.print_endline
