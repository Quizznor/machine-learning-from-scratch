template <typename T> struct Vector;
template <typename M> struct Matrix 
{
    int n_rows, n_cols;
    std::vector<Vector<M>> matrix;

    // init by size and all random values
    Matrix(int size_x, int size_y)
    {
        n_cols = size_x;
        n_rows = size_y;

        for (int i = 0; i < n_rows; i++) 
        {  
            Vector<M> Row(n_cols);
            matrix.push_back(Row);
        }
    }

    // init by size and initial value
    Matrix(int size_x, int size_y, M initial_value)
    {
        n_cols = size_x;
        n_rows = size_y;

        for (int i = 0; i < n_rows; i++) 
        { 
            Vector<M> Row(n_cols, initial_value);
            matrix.push_back(Row);
        }
    }

    // init by values (automatic sizing)
    Matrix(std::vector<Vector<M>> values)
    {
        n_cols = values[0].n_rows;
        n_rows = values.size();
        matrix = values;
    }

    // reset value values of buffer to zero
    void reset_buffer() 
    {
        for (auto &Row : matrix){Row.reset_buffer();}
    }

    // return vector containing sum of the rows
    Vector<M> rowsum()
    {
        std::vector<M> result(n_rows, 0);

        for (int row = 0; row < n_rows; row++) 
        {
            for (int col = 0; col < n_cols; col++)
            {
                result[row] += matrix[row].vector[col];
            }
        }

        Vector<M> Result(result);
        return Result;
    }

    // Define matrix substraction
    Matrix<M> operator - (const Matrix<M> That)
    {
        for (int i = 0; i < n_rows; i++) {this->matrix[i] -= That.matrix[i];}

        return *this;
    }

    // Define implicit matrix substraction
    Matrix<M> operator -= (const Matrix<M> That)
    {
        if (this->n_rows != That.n_rows){throw std::runtime_error("Matrix substraction: incompatible size");}
        if (this->n_cols != That.n_cols){throw std::runtime_error("Matrix substraction: incompatible size");}

        for (int i = 0; i < n_rows; i++){this->matrix[i] -= That.matrix[i];}

        return *this;
    }

    // Define implicit matrix addition
    Matrix<M> operator += (const Matrix<M> That)
    {
        if (this->n_rows != That.n_rows){throw std::runtime_error("Matrix substraction: incompatible size");}
        if (this->n_cols != That.n_cols){throw std::runtime_error("Matrix substraction: incompatible size");}

        for (int i = 0; i < n_rows; i++){this->matrix[i] += That.matrix[i];}

        return *this;
    }

    // Define scalar multiplication
    Matrix<M> operator * (const M factor)
    {
        for (auto &Row : matrix){Row = Row * factor;}

        return *this;
    }

    // Define matrix vector product ( n x (m x n) = m )
    Vector<M> operator * (const Vector<M> That)
    {
        if (this->n_cols != That.n_rows) {throw std::runtime_error("Matrix-Vector product: incompatible size ");}

        std::vector<M> result;

        for (int i = 0; i < this->n_rows; i++)
        {
            M sum = (M)0;
            for (int j = 0; j < this->n_cols; j++)
            {
                sum += matrix[i].vector[j] * That.vector[i];
            }
            result.push_back(sum);
        }

        Vector<M> Result(result);
        return Result;
    }

};