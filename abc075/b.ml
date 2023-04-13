open Base;;

let h, w = Caml.Scanf.sscanf (Caml.read_line ()) "%d %d" (fun h w -> (h, w)) in
let s = List.range 0 h |> List.map ~f:(fun _ -> Caml.read_line ()) in
let answer =
  let a = List.map s ~f:String.to_array |> List.to_array in
  let f i j =
    List.cartesian_product [ -1; 0; 1 ] [ -1; 0; 1 ]
    |> List.map ~f:(fun (di, dj) -> (i + di, j + dj))
    |> List.count ~f:(fun (i, j) ->
           i >= 0 && i < h && j >= 0 && j < w && Char.(a.(i).(j) = '#'))
    |> Int.to_string
  in
  Array.mapi a ~f:(fun i row ->
      Array.mapi row ~f:(fun j c -> if Char.(c = '#') then "#" else f i j)
      |> Array.fold ~init:"" ~f:( ^ ))
  |> Array.to_list
in
answer |> List.iter ~f:Caml.print_endline
