pub fn print_midi(midi_pos: Vec<i32>) {
    let items = get_display_items(midi_pos);
    display(items);
}

fn get_display_items(midi_pos: Vec<i32>) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let map = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];

    for s in midi_pos {
        if s == -1 {
            continue;
        }

        let octave = (s / 12) -1;
        let offset = s % 12;

        let item_pre = map[offset as usize].to_string();
        let item_post = octave.to_string();
        let item = item_pre+&item_post;

        result.push(item);
    }

    result
}

fn display(items: Vec<String>) {
    let len = items.len();

    for s in items {
        print!("{} ", s);
    }
    if len != 0 { println!() };
}
