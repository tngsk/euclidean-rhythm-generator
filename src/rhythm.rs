/// ユークリッドアルゴリズムを使用してリズムパターンを生成する関数
pub fn euclidean_rhythm(pulses: usize, steps: usize) -> Vec<bool> {
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

    // ビョルクルンドのアルゴリズム
    let mut groups: Vec<Vec<bool>> = vec![vec![true]; pulses];
    let mut remainders: Vec<Vec<bool>> = vec![vec![false]; steps - pulses];

    while remainders.len() > 1 {
        let min_len = std::cmp::min(groups.len(), remainders.len());

        for i in 0..min_len {
            groups[i].extend(remainders[i].clone());
        }

        if remainders.len() <= groups.len() {
            let mut next_remainders = Vec::new();
            for i in min_len..groups.len() {
                next_remainders.push(groups[i].clone());
            }
            groups.truncate(min_len);
            remainders = next_remainders;
        } else {
            let mut next_remainders = Vec::new();
            for i in min_len..remainders.len() {
                next_remainders.push(remainders[i].clone());
            }
            remainders = next_remainders;
        }
    }

    let mut result = Vec::with_capacity(steps);
    for group in groups {
        result.extend(group);
    }
    for remainder in remainders {
        result.extend(remainder);
    }

    result
}

pub fn rotate_rhythm(rhythm: &[bool], rotation: usize) -> Vec<bool> {
    let len = rhythm.len();
    if len == 0 {
        return vec![];
    }
    let rotation = rotation % len;
    if rotation == 0 {
        return rhythm.to_vec();
    }

    // 右シフト（音楽的な遅延）を実装
    let split_pos = len - rotation;
    let mut result = Vec::with_capacity(len);
    result.extend_from_slice(&rhythm[split_pos..]);
    result.extend_from_slice(&rhythm[..split_pos]);
    result
}

/// リズムパターンを視覚的な文字列に変換する関数
/// true = "x" (オンセット)
/// false = "." (休符)
pub fn rhythm_to_string(rhythm: &[bool]) -> String {
    rhythm
        .iter()
        .map(|&x| if x { "x" } else { "." })
        .collect::<Vec<_>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclidean_rhythms() {
        assert_eq!(rhythm_to_string(&euclidean_rhythm(3, 8)), "x..x..x.");
        assert_eq!(rhythm_to_string(&euclidean_rhythm(5, 8)), "x.xx.xx.");
        assert_eq!(rhythm_to_string(&euclidean_rhythm(5, 16)), "x..x..x..x..x...");
        assert_eq!(rhythm_to_string(&euclidean_rhythm(7, 16)), "x..x.x.x..x.x.x.");
    }
}
