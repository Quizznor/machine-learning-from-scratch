template <typename M> struct Matrix;
template <typename T> struct Vector {

    int n_rows;
    std::vector<T> vector;

    // init by size and random value (-5,5)
    Vector(int size) 
    {
        for (int i = 0; i < size; i++)
        {
            T random_value = 10 * ( ((T)std::rand()/RAND_MAX) - 0.5);
            vector.push_back(random_value);
        }
        n_rows = size;
    }

    // init by size and initial value
    Vector(int size, T initial_value)
    {
        vector = std::vector<T>(size, (T)initial_value);
        n_rows = size;
    }

    // init by values (automatic sizing)
    Vector(std::vector<T> values) 
    {
        n_rows = values.size();
        vector = values;
    }

    // reset value values of buffer to zero
    void reset_buffer() 
    {
        for (auto &value : vector){value = (T)0;}
    }

    // Define inner product ( n x n = scalar ) 
    T operator * (const Vector<T> That)
    {
        if (n_rows != That.n_rows) {throw std::runtime_error("VECTOR INNER PRODUCT: incompatible size ");}
        
        T sum = (T)0;

        for (int n = 0; n < n_rows; n++) {sum += this->vector[n] * That.vector[n];}

        return sum;
    }

    // Define outer product ( n x m = ( n x m ) )
    Matrix<T> operator & (const Vector<T> That)
    {
        std::vector<Vector<T>> result(this->n_rows, Vector<T>(That.n_rows));

        for (int n = 0; n < this->n_rows; n++) 
        {
            for (int m = 0; m < That.n_rows; m++) 
            {
                result[n].vector[m] = this->vector[n] * That.vector[m];
            }
        }

        Matrix<T> Result(result);

        return Result;
    }

    // Define vector addition
    Vector<T> operator + (const Vector That)
    {
        if (this->n_rows != That.n_rows) {throw std::runtime_error("VECTOR ADDITION: incompatible size ");}

        for (int i = 0; i < n_rows; i++) {this->vector[i] = (T)this->vector[i] + (T)That.vector[i];}

        return *this;
    }

    // Define vector subtraction
    Vector<T> operator - (const Vector That)
    {
        if (this->n_rows != That.n_rows) {throw std::runtime_error("VECTOR ADDITION: incompatible size ");}

        for (int i = 0; i < n_rows; i++) {this->vector[i] = (T)this->vector[i] - (T)That.vector[i];}

        return *this;
    }

    // Define scalar multiplication
    Vector<T> operator * (const T factor)
    {
        for (auto &value : this->vector){value *= factor;}

        return *this;
    }

    // Define implicit vector substraction
    Vector<T> operator -= (const Vector That)
    {
        if (this->n_rows != That.n_rows) {throw std::runtime_error("VECTOR SUBSTRACTION: incompatible size ");}

        for (int i = 0; i < n_rows; i++) {this->vector[i] -= That.vector[i];}

        return *this;
    }

    // Define implicit vector addition
    Vector<T> operator += (const Vector That)
    {
        if (this->n_rows != That.n_rows) {throw std::runtime_error("VECTOR SUBSTRACTION: incompatible size ");}

        for (int i = 0; i < n_rows; i++) {this->vector[i] += That.vector[i];}

        return *this;
    }

    // allow conversion (from T) to double, needed for input layer
    static Vector<double> copy_and_normalize(const Vector<T> Image)
    {
        Vector<double> Result(Image.n_rows);

        for (int i = 0; i < Image.n_rows; i++) {Result.vector[i] = (double)Image.vector[i];}

        return Result;
    }

    // allow appending values to the end of vector, needed for matrix class
    void push_back(T value) {this->vector.push_back(value);}

    // print contents of vector to std::cout
    void print_vector() {

        for (const auto &value : vector) 
        {
            std::cout << std::fixed << std::showpoint;
            std::cout << std:: setprecision(2);
            std::cout << value << " ";
        }
    }

};