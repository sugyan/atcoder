open Base;;

let n, y = Caml.Scanf.sscanf (Caml.read_line ()) "%d %d" (fun n y -> (n, y)) in
let answer =
  let a = ref (-1, -1, -1) in
  for i = 0 to n do
    for j = 0 to n do
      let k = n - i - j in
      if k >= 0 && (10000 * i) + (5000 * j) + (1000 * k) = y then a := (i, j, k)
    done
  done;
  !a
in
answer
|> (fun (a, b, c) -> Printf.sprintf "%d %d %d" a b c)
|> Caml.print_endline
