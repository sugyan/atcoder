open Base;;

let f _ = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun x y -> (x, y)) in
let n, m = f () in
let a =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let bc = List.range 0 m |> List.map ~f in
let answer =
  let f acc (b, c) =
    Map.update acc c ~f:(function None -> b | Some v -> v + b)
  in
  List.map a ~f:(fun x -> (1, x))
  |> List.append bc
  |> List.fold ~init:(Map.empty (module Int)) ~f
  |> Map.to_alist ~key_order:`Decreasing
  |> List.fold_until ~init:(0, 0)
       ~f:(fun (acc, c) (k, v) ->
         if c + v >= n then Stop (acc + (k * (n - c)), n)
         else Continue (acc + (k * v), c + v))
       ~finish:Fn.id
  |> fst
in
answer |> Int.to_string |> Stdlib.print_endline
