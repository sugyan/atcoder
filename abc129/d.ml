open Base;;

let h, w =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun h w -> (h, w))
in
let s = List.range 0 h |> List.map ~f:(fun _ -> Stdlib.read_line ()) in
let answer =
  let s = List.to_array s in
  let m = Array.make_matrix ~dimx:h ~dimy:w 0 in
  let f =
    let rec g acc = function
      | [] -> ()
      | (i, j) :: tl ->
          let c = if Char.(s.(i).[j] = '.') then acc + 1 else 0 in
          m.(i).(j) <- m.(i).(j) + c;
          g c tl
    in
    g 0
  in
  let rh = List.range 0 h in
  let rw = List.range 0 w in
  let fh l = List.iter rh ~f:(fun i -> List.map l ~f:(fun j -> (i, j)) |> f) in
  let fw l = List.iter rw ~f:(fun j -> List.map l ~f:(fun i -> (i, j)) |> f) in
  List.iter [ rw; List.rev rw ] ~f:fh;
  List.iter [ rh; List.rev rh ] ~f:fw;
  (Array.to_list m |> Array.concat |> Array.fold ~init:0 ~f:max) - 3
in
answer |> Int.to_string |> Stdlib.print_endline
