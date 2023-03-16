open Base;;

let n, _ = Caml.Scanf.sscanf (Caml.read_line ()) "%d %d" (fun n d -> (n, d)) in
let x =
  List.range 0 n
  |> List.map ~f:(fun _ ->
         Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string)
in
let answer =
  let x = List.to_array x in
  List.cartesian_product (List.range 0 n) (List.range 0 n)
  |> List.filter ~f:(fun (i, j) -> i < j)
  |> List.count ~f:(fun (i, j) ->
         List.zip_exn x.(i) x.(j)
         |> List.fold ~init:0 ~f:(fun acc (a, b) -> acc + ((a - b) * (a - b)))
         |> fun x -> List.range 1 (x + 1) |> List.exists ~f:(fun y -> y * y = x))
in
answer |> Int.to_string |> Caml.print_endline
