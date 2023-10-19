open Base;;

let n = Stdlib.read_int () in
let answer =
  let m = Array.make_matrix ~dimx:10 ~dimy:10 0 in
  let f i =
    let rec g j = if j < 10 then j else g (j / 10) in
    let h, l = (g i, i % 10) in
    m.(h).(l) <- m.(h).(l) + 1
  in
  List.range 1 (n + 1) |> List.iter ~f;
  List.cartesian_product (List.range 0 10) (List.range 0 10)
  |> List.sum (module Int) ~f:(fun (i, j) -> m.(i).(j) * m.(j).(i))
in
answer |> Int.to_string |> Stdlib.print_endline
