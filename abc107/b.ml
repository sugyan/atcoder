open Base;;

let h, w = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun h w -> (h, w)) in
let a = List.range 0 h |> List.map ~f:(fun _ -> Stdlib.read_line ()) in
let answer =
  let a = List.to_array a in
  let r, c = (Array.create ~len:h false, Array.create ~len:w false) in
  List.cartesian_product (List.range 0 h) (List.range 0 w)
  |> List.iter ~f:(fun (i, j) ->
         if Char.(a.(i).[j] = '#') then (
           r.(i) <- true;
           c.(j) <- true));
  let rs = List.range 0 h |> List.filter ~f:(Array.get r) in
  let cs = List.range 0 w |> List.filter ~f:(Array.get c) in
  List.map rs ~f:(fun i ->
      List.map cs ~f:(String.get a.(i)) |> String.of_char_list)
in
answer |> List.iter ~f:Stdlib.print_endline
