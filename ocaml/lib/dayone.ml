let file = "/home/callum/projects/aoc2024/inputs/one.txt"

let read_file name =
    let ic = open_in name in
    let length = in_channel_length ic in
    let content = really_input_string ic length in
    close_in ic;
    content

let f = read_file file

let parse_line line =
    match String.split_on_char ' ' (String.trim line) with
    | [left; right] -> (int_of_string left, int_of_string right)
    | _ -> failwith ("Invalid line format" ^ line)

let lines = String.split_on_char '\n' f

let (left_list, right_list) =
    List.fold_right (fun line (left_acc, right_acc) ->
            let (left, right) = parse_line line in
            (left :: left_acc, right :: right_acc)
        )
        lines
        ([], [])

let run =
    Printf.printf "left list: [";
    List.iter (Printf.printf "%d"; ) left_list;
    Printf.printf "]\n";

    Printf.printf "right list: [";
    List.iter (Printf.printf "%d"; ) right_list;
    Printf.printf "]\n";

