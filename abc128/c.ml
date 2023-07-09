open Base;;

let n, m = Caml.Scanf.sscanf (Caml.read_line ()) "%d %d" (fun n m -> (n, m)) in
let f _ =
  Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let k = List.range 0 m |> List.map ~f |> List.map ~f:List.tl_exn in
let p = f () in
let answer =
  let f i =
    let g j = (i lsr (j - 1)) land 1 = 1 in
    List.zip_exn k p
    |> List.for_all ~f:(fun (k, p) -> List.count k ~f:g % 2 = p)
  in
  List.range 0 (1 lsl n) |> List.count ~f
in
answer |> Int.to_string |> Caml.print_endline
