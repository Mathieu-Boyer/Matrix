use crate::linear_algebra::traits::Field;
use crate::matrix::core::Matrix;
impl <K : Field, const N : usize> Matrix<K , N , N>{

    fn det1x1(&self)->K{
        self.data[0][0]
    }

    fn det2x2(&self)->K{
        let data = &self.data;
        let ad = data[0][0] * data[1][1];
        let bc = data[0][1] * data[1][0];
        ad - bc
    }

    fn det3x3(&self)->K{
        let data = &self.data;
        let a  = data[0][0];
        let b  = data[0][1];
        let c  = data[0][2];
        let ei = data[1][1] * data[2][2];
        let fh = data[1][2] * data[2][1];
        let di = data[1][0] * data[2][2];
        let fg = data[1][2] * data[2][0];
        let dh = data[1][0] * data[2][1];
        let eg = data[1][1] * data[2][0];

        a * (ei - fh) - b * (di - fg) + c * (dh -eg)
    }


    fn det4x4(&self)->K{
        let data = &self.data;

        let minor0 = Matrix::new([
            [data[1][1], data[1][2], data[1][3]],
            [data[2][1], data[2][2], data[2][3]],
            [data[3][1], data[3][2], data[3][3]],
        ]);

        let minor1 = Matrix::new([
            [data[1][0], data[1][2], data[1][3]],
            [data[2][0], data[2][2], data[2][3]],
            [data[3][0], data[3][2], data[3][3]],
        ]);

        let minor2 = Matrix::new([
            [data[1][0], data[1][1], data[1][3]],
            [data[2][0], data[2][1], data[2][3]],
            [data[3][0], data[3][1], data[3][3]],
        ]);

        let minor3 = Matrix::new([
            [data[1][0], data[1][1], data[1][2]],
            [data[2][0], data[2][1], data[2][2]],
            [data[3][0], data[3][1], data[3][2]],
        ]);

        let a = data[0][0];
        let b = data[0][1];
        let c = data[0][2];
        let d = data[0][3];
         a * minor0.det3x3() - b * minor1.det3x3() + c * minor2.det3x3() - d * minor3.det3x3()
    }

    pub fn determinant(&self)->K{
        
        match N {
            0 => K::one(),
            1 => self.det1x1(),
            2 => self.det2x2(),
            3 => self.det3x3(),
            4 => self.det4x4(),
            _ => panic!("Error, the function is only able to work with matrices of size NxN where N <= 4")
        }

    }
}