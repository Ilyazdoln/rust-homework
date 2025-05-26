const HEIGHT: usize = 7;
const WIDTH: usize = 7;

pub fn draw_envelope() {
    let mut output = String::new();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let mid = WIDTH / 2;

            let condition = 
                x + y == mid ||                // /
                x == y + mid ||                // \
                y == x + mid ||                // /
                x + y == WIDTH + mid - 1 ||    // \
                y == 0 || y == HEIGHT - 1 ||   // top & bottom
                x == 0 || x == WIDTH - 1;      // sides

            output.push(if condition { '*' } else { ' ' });
        }
        output.push('\n');
    }

    print!("{}", output);
}