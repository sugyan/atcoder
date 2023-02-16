open Base;;

let m =
  let a =
    List.range 0 3
    |> List.map ~f:(Fn.compose Caml.read_line ignore)
    |> List.map ~f:(String.split ~on:' ')
    |> List.map ~f:(List.map ~f:Int.of_string)
  in
  let n = Caml.read_int () in
  let b = List.range 0 n |> List.map ~f:(Fn.compose Caml.read_int ignore) in
  let f x = List.exists b ~f:(( = ) x) in
  a |> List.map ~f:(List.map ~f) |> List.map ~f:Array.of_list |> Array.of_list
in
let solve _ =
  let all f = List.range 0 3 |> List.for_all ~f in
  [
    List.range 0 3 |> List.exists ~f:(fun i -> all (fun j -> m.(i).(j)));
    List.range 0 3 |> List.exists ~f:(fun i -> all (fun j -> m.(j).(i)));
    all (fun i -> m.(i).(i));
    all (fun i -> m.(i).(2 - i));
  ]
  |> List.exists ~f:Fn.id
in
solve () |> (function true -> "Yes" | false -> "No") |> Stdio.print_endline
