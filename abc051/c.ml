open Base;;

let sx, sy, tx, ty =
  Stdlib.Scanf.sscanf (Stdlib.read_line ()) "%d %d %d %d" (fun sx sy tx ty ->
      (sx, sy, tx, ty))
in
let answer =
  let f, dx, dy = (String.make, tx - sx, ty - sy) in
  [
    f dy 'U';
    f dx 'R';
    f dy 'D';
    f dx 'L';
    f 1 'L';
    f (dy + 1) 'U';
    f (dx + 1) 'R';
    f 1 'D';
    f 1 'R';
    f (dy + 1) 'D';
    f (dx + 1) 'L';
    f 1 'U';
  ]
  |> String.concat ~sep:""
in
answer |> Stdlib.print_endline
