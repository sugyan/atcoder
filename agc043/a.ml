open Base;;

let h, w = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun h w -> (h, w)) in
let s = List.range 0 h |> List.map ~f:(fun _ -> Stdlib.read_line ()) in
let answer =
  let s = List.to_array s in
  let m = Array.make_matrix ~dimx:h ~dimy:w (h + w) in
  m.(0).(0) <- Bool.to_int Char.(s.(0).[0] = '#');
  for i = 0 to h - 1 do
    for j = 0 to w - 1 do
      let f (di, dj) =
        if i + di >= 0 && j + dj >= 0 then
          let d =
            Bool.to_int Char.(s.(i + di).[j + dj] = '.' && s.(i).[j] = '#')
          in
          m.(i).(j) <- min m.(i).(j) (m.(i + di).(j + dj) + d)
      in
      List.iter [ (-1, 0); (0, -1) ] ~f
    done
  done;
  m.(h - 1).(w - 1)
in
answer |> Int.to_string |> Stdlib.print_endline
