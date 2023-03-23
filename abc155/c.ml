open Base;;

let n = Caml.read_int () in
let s = List.range 0 n |> List.map ~f:(fun _ -> Caml.read_line ()) in
let answer =
  let g = List.sort s ~compare:Poly.compare |> List.group ~break:Poly.( <> ) in
  let m = List.map g ~f:List.length |> List.fold ~init:0 ~f:max in
  List.filter_map g ~f:(fun l ->
      if List.length l = m then Some (List.hd_exn l) else None)
in
answer |> List.iter ~f:Caml.print_endline
