use crate::vector::core::Vector;
use crate::linear_algebra::traits::Field;
use crate::matrix::core::Matrix;

impl<K : Field ,const N: usize> Vector<K,N> {
    pub fn display(&self){
        print!("Size : {} , Content : [ ", self.size());
        for i in 0..N{
            print!("{}", self.data[i]);
            if i < N - 1{print!(", ")}
        }
        print!(" ]\n");
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn to_matrix<const COLUMN: usize, const ROW: usize>(&self) -> Matrix<K, COLUMN, ROW> where K : Copy {

        if (COLUMN * ROW) > self.size(){ panic!("Error : Invalid Shape Resize Value.");}

        let mut data : [[K; ROW]; COLUMN] = [[K::default(); ROW]; COLUMN];
        for i in 0..COLUMN {
            for j in 0..ROW {
                data[i][j] = self.data[(i * COLUMN) + j];
            }
        }
        Matrix { data }
    }
}