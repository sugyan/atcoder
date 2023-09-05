open Base;;

let n = Stdlib.read_int () in
let s = List.range 0 n |> List.map ~f:(fun _ -> Stdlib.read_line ()) in
let answer =
  let a2z = List.init 26 ~f:(fun i -> Char.(of_int_exn (i + to_int 'a'))) in
  let f s = a2z |> List.map ~f:(fun c -> String.count s ~f:(Char.( = ) c)) in
  List.map s ~f
  |> List.fold ~init:(List.init 26 ~f:(Fn.const 50)) ~f:(List.map2_exn ~f:min)
  |> Fn.flip List.zip_exn a2z
  |> List.map ~f:(fun (i, c) -> String.make i c)
  |> String.concat ~sep:""
in
answer |> Stdlib.print_endline
