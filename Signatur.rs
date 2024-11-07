fn count_permutation(shipments: &Vec<u32>) -> usize {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len();

    if total % n as u32 != 0 {
        return 0;
    }

    let target = total / n as u32;
    let mut moves = 0;
    let mut excess = 0;

    for &shipment in shipments {
        if shipment > target {
            excess += shipment - target;
        } else {
            moves += target - shipment;
        }
    }

    moves
}

fn can_distribute_equally(shipments: &Vec<u32>) -> bool {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len();

    total % n as u32 == 0
}

fn gen_shipments(n: usize) -> Vec<u32> {
    let mut shipments = vec![1; n];
    let total = n as u32;
    let target = total / n as u32;

    for i in 0..n {
        shipments[i] = target;
    }

    shipments
}

fn main() {
    let shipments = vec![10, 20, 30, 40, 50];
    
    if can_distribute_equally(&shipments) {
        let moves = count_permutation(&shipments);
        println!("Мінімальна кількість переміщень: {}", moves);
    } else {
        println!("Неможливо рівномірно розподілити вантаж");
    }

    let equal_shipments = gen_shipments(5);
    println!("Рівномірно розподілені вантажі: {:?}", equal_shipments);
}
