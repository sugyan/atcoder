let n = read_float () in
let x = read_line () |> String.split_on_char ' ' |> List.map int_of_string in
let m =
  let avg = List.fold_left ( + ) 0 x |> float_of_int |> Fun.flip ( /. ) n in
  floor (avg +. 0.5) |> int_of_float
in
List.map (fun x -> (x - m) * (x - m)) x
|> List.fold_left ( + ) 0 |> Printf.printf "%d"
