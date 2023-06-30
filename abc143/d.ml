open Base;;

let n = Caml.read_int () in
let l =
  Caml.read_line () |> String.split ~on:' ' |> List.map ~f:Int.of_string
in
let answer =
  let a = List.sort l ~compare |> List.to_array in
  let how = `First_greater_than_or_equal_to in
  let f i =
    let g j =
      Array.binary_search ~pos:(j + 1) a ~compare how (a.(i) + a.(j))
      |> Option.value ~default:n
      |> ( + ) (-j - 1)
    in
    List.range (i + 1) (n - 1) |> List.sum (module Int) ~f:g
  in
  List.range 0 (n - 2) |> List.sum (module Int) ~f
in
answer |> Int.to_string |> Caml.print_endline
