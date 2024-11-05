/// ユークリッドアルゴリズムを使用してリズムパターンを生成する関数
fn euclidean_rhythm(pulses: usize, steps: usize) -> Vec<bool> {
    // パラメータのバリデーション
    if pulses > steps {
        return vec![];
    }
    if pulses == 0 {
        return vec![false; steps]; // 全て休符
    }
    if pulses == steps {
        return vec![true; steps]; // 全てオンセット
    }

    // 初期シーケンスの生成 (true=オンセット, false=休符)
    let mut sequence = Vec::with_capacity(steps);
    sequence.extend(vec![true; pulses]);
    sequence.extend(vec![false; steps - pulses]);

    // 各要素を単一の要素を持つベクターに変換
    let mut bucket: Vec<Vec<bool>> = sequence.into_iter().map(|x| vec![x]).collect();
    // [true, true, true, true, true, false, false, false] から
    // [[true], [true], [true], [true], [true], [false], [false], [false]] になる

    // バケットが2つ以下になるまでグループ化を繰り返す
    // 1. 初期状態:
    // bucket = [[t], [t], [t], [t], [t], [f], [f], [f]]
    while bucket.len() > 2 {
        let mut next_bucket = Vec::new();
        let remainder = bucket.len() % 2;
        let pairs = bucket.len() / 2;

        // 隣接するグループを結合
        for i in 0..pairs {
            let mut group = bucket[i].clone();
            group.extend(bucket[pairs + i].clone());
            next_bucket.push(group);
        }

        // 余りのグループがある場合は追加
        if remainder == 1 {
            next_bucket.push(bucket[bucket.len() - 1].clone());
        }

        bucket = next_bucket;

        // 2. 1回目のループ:
        // pairs = 4
        // bucket[0] + bucket[4] = [t,t]
        // bucket[1] + bucket[5] = [t,f]
        // bucket[2] + bucket[6] = [t,f]
        // bucket[3] + bucket[7] = [t,f]
        // next_bucket = [[t,t], [t,f], [t,f], [t,f]]
        // 3. 2回目のループ:
        // pairs = 2
        // bucket[0] + bucket[2] = [t,t,t,f]
        // bucket[1] + bucket[3] = [t,f,t,f]
        // next_bucket = [[t,t,t,f], [t,f,t,f]]
    }

    // 最終的なリズムパターンの生成
    let mut result = bucket[0].clone();
    if bucket.len() > 1 {
        result.extend(bucket[1].clone());
    }

    // 4. 最終結合:
    // result = [t,.,t,t,.,t,t,.]  // [x.xx.xx.]

    result
}

/// リズムパターンを指定した数だけ回転させる関数
fn rotate_rhythm(rhythm: &[bool], rotation: usize) -> Vec<bool> {
    let len = rhythm.len();
    if len == 0 {
        return vec![];
    }
    let rotation = rotation % len;
    let mut result = Vec::with_capacity(len);
    result.extend_from_slice(&rhythm[rotation..]);
    result.extend_from_slice(&rhythm[..rotation]);
    result
}

/// リズムパターンを視覚的な文字列に変換する関数
/// true = "x" (オンセット)
/// false = "." (休符)
fn rhythm_to_string(rhythm: &[bool]) -> String {
    rhythm
        .iter()
        .map(|&x| if x { "x" } else { "." })
        .collect::<Vec<_>>()
        .join("")
}

fn main() {
    // テスト用のリズムパターン例
    let examples = [
        (3, 8),  // Cuban tresillo
        (5, 8),  // Cuban cinquillo
        (5, 16), // Bossa-nova
        (7, 16), // Brazilian Samba
    ];

    // 各リズムパターンの生成と表示
    for (pulses, steps) in examples {
        let rhythm = euclidean_rhythm(pulses, steps);
        println!("E({}, {}) = [{}]", pulses, steps, rhythm_to_string(&rhythm));

        // 全ての回転パターンを表示
        for i in 1..steps {
            let rotated = rotate_rhythm(&rhythm, i);
            println!("  Rotation {}: [{}]", i, rhythm_to_string(&rotated));
        }
        println!("");
    }
}
