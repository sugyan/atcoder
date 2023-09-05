open Base;;

let n = Stdlib.read_int () in
let s = List.range 0 n |> List.map ~f:(fun _ -> Stdlib.read_line ()) in
let answer =
  let f c = List.count s ~f:(fun s -> Char.(s.[0] = c)) in
  let a = Array.map [| 'M'; 'A'; 'R'; 'C'; 'H' |] ~f in
  let sum = ref 0 in
  for i = 0 to 2 do
    for j = i + 1 to 3 do
      for k = j + 1 to 4 do
        sum := !sum + (a.(i) * a.(j) * a.(k))
      done
    done
  done;
  !sum
in
answer |> Int.to_string |> Stdlib.print_endline
