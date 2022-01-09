#include <algorithm>
#include <iostream>
#include <iomanip>
#include <fstream>
#include <numeric>
#include <random>
#include <vector>
#include <chrono>
#include <cmath>

// specify dataset and target output
const int image_height = 28;                    // image height (pixel)
const int image_width  = 28;                    // image width  (pixel)
const int output_size  = 10;                    // map to digits 0 to 9
const int max_pixel   = 255;                    // max pixel brightness

// header files for data structures
#include "Vector.h"                             // provides Vector<T>
#include "Matrix.h"                             // provides Matrix
#include "NLayer.h"                             // provides NLayer     
#include "Dataset.h"                            // provides Dataset                       
#include "NNetwork.h"                           // provides NNetwork

// placeholder variables

int main() 
{

    Dataset TrainingSet;
    TrainingSet.read_MNIST("dataset/train-images-idx3-ubyte");     // read images
    TrainingSet.read_MNIST("dataset/train-labels-idx1-ubyte");     // read labels

    NNetwork DigitClassifier({image_width * image_height, 16, 16, output_size});
    DigitClassifier.load_training_set(&TrainingSet);
    DigitClassifier.train(0.5);
    // DigitClassifier.save_parameters("parameters");

    return 0;
}