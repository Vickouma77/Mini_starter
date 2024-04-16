//Pascal's Triangle in Rust

fn main() {
    let rows = 10;
    let mut triangle: Vec<Vec<u64>> = Vec::with_capacity(rows);

    for i in 0..rows {
        let mut row = Vec::with_capacity(i + 1);

        row.push(1);

        for j in 1..i {
            let val = triangle[i - 1][j - 1] + triangle[i - 1][j];
            row.push(val);
        }

        if i > 0 {
            row.push(1);
        }

        triangle.push(row);
    }

    for row in triangle.iter() {
        println!("{:?}", row);
    }
}

//Output
/* [1]
[1, 1]
[1, 2, 1]
[1, 3, 3, 1]
[1, 4, 6, 4, 1]
[1, 5, 10, 10, 5, 1]
[1, 6, 15, 20, 15, 6, 1]
[1, 7, 21, 35, 35, 21, 7, 1]
[1, 8, 28, 56, 70, 56, 28, 8, 1]
[1, 9, 36, 84, 126, 126, 84, 36, 9, 1]
 */
