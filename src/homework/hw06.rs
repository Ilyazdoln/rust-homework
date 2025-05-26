pub fn draw_tree(triangle_count: usize) {
    let mut output = String::new();

    (0..triangle_count).for_each(|t| {
        let base_width = 5;
        let height = 3;

        (0..height).for_each(|i| {
            let stars = 1 + i * 2;
            let spaces = (base_width - stars) / 2;
            let line = " ".repeat(spaces) + &"*".repeat(stars);
            output.push_str(&line);
            output.push('\n');
        });
    });

    print!("{}", output);
}