open Base;;

let n, m = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun n m -> (n, m)) in
let a =
  let f _ =
    Stdlib.read_line () |> String.split ~on:' ' |> List.tl_exn
    |> List.map ~f:Int.of_string
  in
  List.range 0 n |> List.map ~f
in
let answer =
  let b = Array.init m ~f:(Fn.const 0) in
  List.iter a ~f:(List.iter ~f:(fun i -> b.(i - 1) <- b.(i - 1) + 1));
  Array.count b ~f:(( = ) n)
in
answer |> Int.to_string |> Stdlib.print_endline
