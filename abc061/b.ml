open Base;;

let n, m = Caml.Scanf.sscanf (Caml.read_line ()) "%d %d" (fun n m -> (n, m)) in
let ab =
  List.range 0 m
  |> List.map ~f:(fun _ ->
         Caml.Scanf.sscanf (Caml.read_line ()) "%d %d" (fun a b -> (a, b)))
in
let answer =
  let c = Array.init n ~f:(Fn.const 0) in
  List.iter ab ~f:(fun (a, b) ->
      c.(a - 1) <- c.(a - 1) + 1;
      c.(b - 1) <- c.(b - 1) + 1);
  Array.to_list c
in
answer |> List.iter ~f:(Fn.compose Caml.print_endline Int.to_string)
