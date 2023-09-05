open Base;;

let n = Stdlib.read_int () in
let s = Stdlib.read_line () in
let answer =
  let cs = List.init 26 ~f:(fun i -> Char.of_int_exn (i + 97)) in
  let ok = List.mem ~equal:Char.equal in
  List.init n ~f:(fun i -> String.to_list s |> Fn.flip List.split_n i)
  |> List.map ~f:(fun (l, r) -> List.count cs ~f:(fun c -> ok l c && ok r c))
  |> List.fold ~init:0 ~f:max
in
answer |> Int.to_string |> Stdlib.print_endline
