const W: usize = 20;
const H: usize = 10;

fn main() {
    // Зберемо весь малюнок у один String, щоб потім вивести print! один раз
    let mut envelope = String::new();

    for row in 0..H {
        for col in 0..W {
            // Верхній і нижній рядок
            if row == 0 || row == H - 1 {
                if col == 0 || col == W - 1 {
                    envelope.push('+');
                } else {
                    envelope.push('-');
                }
            }
            // Бокові рядки з "клапанами"
            else {
                if col == 0 || col == W - 1 {
                    envelope.push('|');
                } else {
                    // Розрахунок діагоналей
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