// 3. Чи завжди можливо забезпечити однаковий вантаж?
// Відповідь: Ні. Якщо сума всіх вантажів не ділиться на кількість кораблів без остачі — це неможливо.

// 4. Як буде виглядати сигнатура в іншому випадку?
// Відповідь: Повертаємо Option<usize> замість usize, щоб позначити, що може бути None (неможливо вирівняти).

pub fn count_permutation(shipments: &Vec<u32>) -> Option<usize> {
    let total: u32 = shipments.iter().sum();
    let len = shipments.len() as u32;

    if total % len != 0 {
        return None;
    }

    let target = total / len;
    let mut moves = 0;

    for &ship in shipments.iter() {
        if ship > target {
            moves += (ship - target) as usize;
        }
    }

    Some(moves)