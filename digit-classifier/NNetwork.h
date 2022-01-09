class NNetwork {

    public:

        int n_layers;                       // number of layers (inc. input)
        int parameters = 0;                 // total parameters of the network
        std::string layout = "";            // architecture of the network

        // init untrained network via layers, nodes
        NNetwork(std::vector<int> nodes) 
        {

            std::srand(time(0));            // set seed for RNGs

            // build the neural network node for node
            for (auto n = nodes.begin(); n < nodes.end(); n++) 
            {
                NLayer::layer_type mode;
                
                if (n==nodes.begin()) 
                {
                    mode=NLayer::input;
                    layout += std::to_string(*n) + ", ";
                }
                else if (n==nodes.end()-1) 
                {
                    mode=NLayer::output;
                    layout += std::to_string(*n);
                }
                else 
                {
                    mode=NLayer::hidden;
                    layout += std::to_string(*n) + ", ";
                }

                NLayer NewLayer(*n,*(n-1), mode);
                parameters += NewLayer.parameters;
                layers.push_back(NewLayer);
            }

            n_layers = nodes.size();

            std::cout << "Finished building the neural network!" << std::endl;
        
        }

        // add reference to training set to classifier
        void load_training_set(Dataset* Data) {TrainingSet = Data;}

        // train the neural network via stochastic gradient descent
        void train(double beta=1e-3, int n_gens=100, int n_batches=100)
        {

            time_t start = std::time(NULL);
            max_gens = n_gens;
            n_batch = n_batches;

            // iterate over n_gens
            for (int generation = 1; generation <= n_gens; generation++) 
            {
                // create shuffled subset of training set
                const auto batches = TrainingSet->create_batches(n_batches);

                current_batch = 1;

                // perform gradient descent over each batch
                for (const auto &batch : batches) 
                {
                    // ensure batch is not completely statistically insignificant
                    if (batch.size() < 30) {continue;}

                    // calculate average weight and bias gradient for batch
                    calculate_batch_gradient(batch);

                    // gradient descent with momentum
                    for (int i = 1; i < layers.size(); i++)
                    {
                        // stochastic gradient descent with momentum
                        // layers[i].Biases = layers[i].Biases * beta - layers[i].PartialBiases * (1 - beta);
                        // layers[i].Weights = layers[i].Weights * beta - layers[i].PartialWeights * (1 - beta);

                        // without momentum
                        layers[i].Biases -= layers[i].PartialBiases;
                        layers[i].Weights -= layers[i].PartialWeights;
                        
                        layers[i].PartialActivation.reset_buffer();
                        layers[i].PartialBiases.reset_buffer();
                        layers[i].PartialWeights.reset_buffer();
                    }

                    time_t stop = std::time(NULL);
                    auto duration = stop - start;

                    std::vector<int> parameters = {generation, n_gens, (int)duration, current_batch, n_batches, n_batch};

                    about_training(parameters, last_n);
                    current_batch++;
                }
            }
        }

        // export neural network parameters to directory
        void save_parameters(std::string dir_name)
        {
            for (int i = 0; i < layers.size(); i++)
            {
                std::string file_name = dir_name + "/layer-" + std::to_string(i);

                std::cout << file_name << " ";

                // std::ofstream layer (file_name);

                // if (layer.is_open()) {std::cout << "success!";}
                // else {std::cout << "failure!";}
            }
        }

    private:

        std::vector<NLayer> layers;                 // container for NLayers
        Dataset* TrainingSet;                       // training set address

        // information for about_training()
        int last_n;                                 // last image in batch
        int n_batch;                                // number of batches for training
        int max_gens;                               // number of training generations
        double avg_cost = 0;                        // average cost function for a batch
        int current_batch = 1;                      // current batch for training
        int last_prediction;                        // last predicted digit of batch during training

        // propagate image over all layers and return resulting neuron values
        void calculate_network_output(Vector<uint8_t> Image) 
        {
            auto Activation = Vector<uint8_t>::copy_and_normalize(Image);
            layers.begin()->RawOutput = Activation;                                 // add input once

            for (auto layer = layers.begin()+1; layer < layers.end(); layer++ ) 
            {
                Activation = layer->Weights * Activation + layer->Biases;           // propagate to next node
                layer->RawOutput = Activation;                                      // copy raw values to layer
                Activation = layer->activation_function(Activation);                // apply normalization
                layer->Output = Activation;                                         // copy normalized values
            }
        }

        // print information about training progress
        void about_training(std::vector<int> p, int last_image)
        {
            // clear the console
            std::cout << "\x1B[2J\x1B[H";

            // update information
            std::cout << "TRAINING IN PROGRESS for network (" << layout << ")";  
            std::cout << "\n";
            std::cout << " ------------------------------------------------------------- \n";
            std::cout << "| Generation:  " << std::setw(4) << p[0] << "/" << p[1] << std::setw(11) << " "
                      << "  Training time: " << std::setw(7) << p[2] << "s" << std::setw(5) << "|\n";
            std::cout << "| Curr. Batch: " << std::setw(4) << p[3] << "/" << p[4] << std::setw(11) << " "
                      << "  Avg. cost:     " << std::setw(8) << std::setprecision(4) << avg_cost 
                      << std::setw(5) << "|\n";
            std::cout << "| Batch size: " << std::setw(4) << p[5] << " imgs" << std::setw(11) << " " 
                      << "  Last guess:      " << std::setw(6) << last_prediction  << std::setw(5) << "|\n";
            std::cout << "|                                                             |\n";
            std::cout << "| Output: "; layers.back().Output.print_vector(); std::cout << "  |\n";
            std::cout << " ------------------------------------------------------------- \n";
            std::cout << std::endl;
           
            TrainingSet->print_image(last_image);
        }

        // weight of partial derivative contributions for each node
        Vector<double> d_cost_function(int label) 
        {
            NLayer& Layer = layers[n_layers - 1];
            Vector<double> Result(Layer.n_this_layer);

            for (int i = 0; i < Layer.n_this_layer; i++) 
            {
                double should_be = (i==label) ? 1 : 0;
                Result.vector[i] = 2 * Layer.activation_function(Layer.RawOutput).vector[i] - should_be;
            }

            return Result;
        }

        // cost function for the network, mean square error
        double cost_function(int label) 
        {
            double sum = 0;
            NLayer& Layer = layers[n_layers - 1];

            for (int i = 0; i <= 9; i++)
            {
                double should_be = (i==label) ? 1 : 0;
                sum += pow(Layer.activation_function(Layer.RawOutput).vector[i] - should_be,2);
            }
        
            return sum;
        }

        // make a prediction based on last image that was fed to NN
        int make_prediction()
        {
            NLayer& OutputLayer = layers[n_layers - 1];
            auto predictions = OutputLayer.Output.vector;

            return std::distance(predictions.begin(), std::max_element(predictions.begin(),predictions.end()));
        }

         // calculate bias gradient for each layer and batch
        void calculate_batch_gradient(const std::vector<int>& batch)
        {
            avg_cost = 0;

            for (const auto &n : batch)
            {
                last_n = n;
                n_batch = batch.size();
                const auto this_image = (TrainingSet->images)[n];
                const auto this_label = (TrainingSet->labels)[n];

                // calculate outputs for this image
                calculate_network_output(this_image);

                avg_cost += 1./batch.size() * cost_function(this_label);
                last_prediction = make_prediction();

                // calculate weights for each layer
                for (int index = layers.size() - 1; index > 0; index--)
                {
                    NLayer& Layer = layers[index];
                    NLayer& PrevLayer = layers[index - 1];
                    NLayer& NextLayer = layers[index + 1];

                    const int& n_this = Layer.n_this_layer;         // row size for this layer
                    const int& n_prev = Layer.n_prev_layer;         // column size for this layer

                    // init containers for derivative
                    Layer.WeightsBuffer.reset_buffer();
                    Layer.BiasesBuffer.reset_buffer();

                    // calculate partial derivative for output layer 
                    if (index == layers.size() - 1)
                    {    
                        Layer.PartialActivation = d_cost_function(this_label);
                        Layer.BiasesBuffer = Layer.PartialActivation;
                    }

                    // perform backpropagation for subsequent layers
                    else 
                    {

                        auto Cost = NextLayer.PartialActivation * NextLayer.d_activation_function(NextLayer.RawOutput);
                        Layer.PartialActivation = (Layer.Weights * Cost).rowsum();

                        Layer.BiasesBuffer = Layer.PartialActivation;
                        Layer.WeightsBuffer = Layer.PartialActivation & PrevLayer.Output;
                    }

                    // add average contribution of image to gradients
                    Layer.PartialBiases += Layer.BiasesBuffer * (1./batch.size());
                    Layer.PartialWeights += Layer.WeightsBuffer * (1./batch.size());
         
                }
            }
        }

};