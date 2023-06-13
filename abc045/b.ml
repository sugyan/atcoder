open Base;;

let s = List.range 0 3 |> List.map ~f:(fun _ -> Caml.read_line ()) in
let answer =
  let s = List.to_array s in
  let a = Array.create ~len:3 0 in
  let rec f i =
    if a.(i) >= String.length s.(i) then Char.(of_int_exn (i + to_int 'A'))
    else
      let c = s.(i).[a.(i)] in
      a.(i) <- a.(i) + 1;
      f Char.(to_int c - to_int 'a')
  in
  f 0
in
answer |> Char.to_string |> Caml.print_endline
