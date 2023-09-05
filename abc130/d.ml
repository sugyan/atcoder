open Base;;

let n, k = Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d" (fun n k -> (n, k)) in
let a =
  Stdlib.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let b = Array.create ~len:(n + 1) 0 in
  List.iteri a ~f:(fun i x -> b.(i + 1) <- b.(i) + x);
  let f i =
    Array.binary_search b ~compare `First_greater_than_or_equal_to (k + b.(i))
    |> function
    | Some j -> n - j + 1
    | None -> 0
  in
  List.range 0 n |> List.sum (module Int) ~f
in
answer |> Int.to_string |> Stdlib.print_endline
