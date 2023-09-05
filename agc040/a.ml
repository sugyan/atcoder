open Base;;

let s = Stdlib.read_line () in
let answer =
  let len = String.length s + 1 in
  let a = Array.create ~len 0 in
  for i = 0 to len - 2 do
    if Char.(s.[i] = '<') then a.(i + 1) <- max a.(i + 1) a.(i) + 1
  done;
  for i = len - 2 downto 0 do
    if Char.(s.[i] = '>') then a.(i) <- max a.(i) (a.(i + 1) + 1)
  done;
  Array.sum (module Int) a ~f:Fn.id
in
answer |> Int.to_string |> Stdlib.print_endline
