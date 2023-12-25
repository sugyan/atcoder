open Base

let () =
  let n = Stdlib.read_int () in
  let read i =
    Int.to_string i |> Stdlib.print_endline;
    Stdlib.read_line ()
  in
  let s0 = read 0 in
  let rec f l h =
    let m = (l + h + 1) / 2 in
    read m |> function
    | "Vacant" -> ()
    | s ->
        if Bool.( = ) String.(s = s0) (m % 2 = 0) then f (m + 1) h
        else f l (m - 1)
  in
  f 0 (n - 1)
