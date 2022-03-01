use rand::Rng;

fn main() {
    println!("random number!");

    const OCCUR_NUM: usize = 100;

    let mut array = [0;OCCUR_NUM];

    // -1000以上1000以下のランダムな整数を10回発生、表示
    for i in 0..OCCUR_NUM {
        let random_num = rand::thread_rng().gen_range(-1000,1001);
        array[i] = random_num;
        println!("array[{}]:{}", i, array[i]);
    }
    // 昇順に並び替え
    let mut j = OCCUR_NUM-1;
    while j > 0 {
        for i in 0..j {
            if array[i] > array[i+1] {
                let big = array[i];
                array[i] = array[i+1];
                array[i+1] = big;
            }
        }
        j -= 1;
    }
    // 昇順で表示
    println!("sort!");
    for i in 0..OCCUR_NUM {
        println!("array[{}]:{}", i, array[i]);
    }
}
