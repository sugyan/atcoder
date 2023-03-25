open Base;;

let n = Caml.read_line () in
let answer =
  let s =
    String.to_list n |> List.map ~f:(fun c -> Char.(to_int c - to_int '0'))
  in
  max
    (List.sum (module Int) s ~f:Fn.id)
    (List.hd_exn s - 1 + (9 * (List.length s - 1)))
in
answer |> Int.to_string |> Caml.print_endline
