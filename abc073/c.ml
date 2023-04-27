open Base;;

let n = Caml.read_int () in
let a = List.range 0 n |> List.map ~f:(fun _ -> Caml.read_int ()) in
let answer =
  let h = Hashtbl.create (module Int) in
  List.iter a ~f:(Hashtbl.incr h);
  Hashtbl.count h ~f:(fun c -> c % 2 = 1)
in
answer |> Int.to_string |> Caml.print_endline
