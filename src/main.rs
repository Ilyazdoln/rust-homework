const W: usize = 20;
const H: usize = 10;

fn main() {
    let mut envelope = String::new();

    for row in 0..H {
        for col in 0..W {
            if row == 0 || row == H - 1 {
                if col == 0 || col == W - 1 {
                    envelope.push('+');
                } else {
                    envelope.push('-');
                }
            } else {
                if col == 0 || col == W - 1 {
                    envelope.push('|');
                } else {
                    let left_diag = col == row;
                    let right_diag = col == W - row - 1;

                    if left_diag {
                        envelope.push('\\');
                    } else if right_diag {
                        envelope.push('/');
                    } else {
                        envelope.push(' ');
                    }
                }
            }
        }
        envelope.push('\n');
    }

    print!("{}", envelope);
    println!();
}