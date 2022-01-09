struct NLayer 
{
    // fix neural layer types
    enum layer_type{input, output, hidden};

    int n_this_layer;                                       // neurons in this layer
    int n_prev_layer;                                       // neurons in previous layer
    int parameters = 0;                                     // parameters in layer
    layer_type layer_mode;                                  // type of layer

    // training parameters
    Vector<double> Biases;                                  // biases for neuron activation
    Matrix<double> Weights;                                 // weight matrix for propagation
    Vector<double> RawOutput;                               // output without activation function
    Vector<double> Output;                                  // output with activation function
    Vector<double> PartialBiases;                           // gradient for bias of neurons
    Matrix<double> PartialWeights;                          // gradient for weights of neurons

    // optimization parameters
    Vector<double> PartialActivation;                       // gradient for node activation
    Vector<double> BiasesBuffer;                            // container for one image derivative
    Matrix<double> WeightsBuffer;                           // container for one image derivative

    NLayer(int n_this, int n_prev, layer_type mode) :

        Biases(Vector<double>(n_this)),
        Weights(Matrix<double>(n_prev, n_this)),
        RawOutput(Vector<double>(n_this, 0)),
        Output(Vector<double>(n_this, 0)),
        PartialBiases(Vector<double>(n_this, 0)),
        PartialWeights(Matrix<double>(n_prev, n_this, 0)),
        PartialActivation(Vector<double>(n_this, 0)),
        BiasesBuffer(Vector<double>(n_this, 0)),
        WeightsBuffer(Matrix<double>(n_prev, n_this, 0))

    {
        n_this_layer = n_this;
        n_prev_layer = n_prev;
        layer_mode = mode;

        // input layer, no weights, biases
        if (mode==input)
        {
            std::cout << "Created input layer (size = " << n_this_layer << ")" << std::endl;
        }

        //hidden or output layer, weights, biases and gradient
        else
        {
            parameters = n_this_layer + (n_this * n_prev);  // (n_this) bias + (n_prev x n_this) weights
            
            if (mode==hidden)
            {
                std::cout << "Created hidden layer: " << n_prev << " -> " << n_this << std::endl;
            }
            if (mode==output)
            {
                std::cout << "Created output layer: " << n_prev << " -> " << n_this << std::endl;
            }
        }
    }

    // mask for the activation function of the layer
    Vector<double> activation_function(Vector<double>& Input) 
    {
        // use ReLU for intermediate layers, use logistic function for output
        if (layer_mode==output) {return logistic_function(Input);}
        else if (layer_mode==input || layer_mode==hidden) {return ReLU(Input);}
    }

    // sigmoid function ( caps between 0 and 1, sigmoid(0) = 0.5 )
    static Vector<double> logistic_function(Vector<double>& Input) 
    {
        std::vector<double> result;

        for (auto &value : Input.vector) {result.push_back(1/(1+std::exp(-value)));}

        Vector<double> Result(result);

        return Result;
    }

    // 0 if activation < 0, returns itself otherwise
    static Vector<double> ReLU(Vector<double>& Input) 
    {
        std::vector<double> result;

        for (auto &value : Input.vector) {result.push_back(std::max(0.,value));}

        Vector<double> Result = Vector<double>(result);

        return Result;
    }

    // derivative of activation function (needed for training)
    Vector<double> d_activation_function(Vector<double> Input)
    {
        if (layer_mode==output) {return d_logistic_function(Input);}
        else if (layer_mode==input || layer_mode==hidden) {return d_ReLU(Input);}
    }

    // derivative of sigmoid
    static Vector<double> d_logistic_function(Vector<double> Input)
    {
        std::vector<double> result;

        for (const auto &value : Input.vector) {result.push_back(std::exp(value)/pow(1+std::exp(value),2) );}

        Vector<double> Result = Vector<double>(result);

        return Result;
    }

    // derivative of ReLU
    static Vector<double> d_ReLU(Vector<double> Input)
    {
        std::vector<double> result;

        for (const auto &value : Input.vector) {result.push_back((value <= 0.) ? 0. : 1. );}

        Vector<double> Result = Vector<double>(result);

        return Result;
    }

};