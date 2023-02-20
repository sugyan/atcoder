open Base;;

let a, b, c =
  Caml.Scanf.sscanf (Caml.read_line ()) "%d %d %d" @@ fun a b c -> (a, b, c)
in
let answer =
  let rec loop (a, b, c) i =
    let odd x = Caml.( mod ) x 2 = 1 in
    if odd a || odd b || odd c then i
    else if a = b && b = c then -1
    else loop ((b + c) / 2, (a + c) / 2, (a + b) / 2) (i + 1)
  in
  loop (a, b, c) 0
in
answer |> Int.to_string |> Stdio.print_endline
