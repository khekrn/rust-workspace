fn pretty_print<const ROW: usize, const COL: usize>(matrix: &[[i32; COL]; ROW]){
    for row in matrix {
        println!("{:?}", row);
    }
}

fn transpose<const ROW: usize, const COL: usize>(matrix: &[[i32; COL]; ROW]) -> [[i32; ROW]; COL] {
    let mut result = [[0; ROW]; COL];
    for i in 0..ROW{
        for j in 0..COL{
            result[j][i] = matrix[i][j];
        }
    }
    return result;
}

fn main(){
    let m1 = [
        [100, 101, 102],
        [200, 201, 202],
        [300, 301, 302],
    ];

    let m2 = [
        [1, 2, 3, 4, 5],
        [5, 6, 7, 8, 10],
        [100, 101, 102, 103, 104],
        [105, 106, 107, 108, 109],
    ];

    pretty_print(&m1);
    println!("Transpose = {:?}\n", transpose(&m1));
    pretty_print(&m2);
    println!("Transpose = {:?}\n", transpose(&m2));
}