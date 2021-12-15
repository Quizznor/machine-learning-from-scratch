
// Nove this out of here as soo as we are done

#include <algorithm>
#include <iostream>
#include <vector>

#include "Node.h"

class DecisionTree {

  public:

    // have to hardcode this at compile time the way it is now =(
    // ideally would maybe change the way dataset is loaded later
    const static int nTrain = 1207, nTest = 213;
    const static int nFeatures = 129;

    std::vector<std::vector<double>> trainingSet {nTrain, std::vector<double> (nFeatures) };
    std::vector<std::vector<double>> testingSet {nTest, std::vector<double> (nFeatures) };

    DecisionTree(int depth) {

        int maxTreeDepth = depth;

        double dataTrain[nTrain][nFeatures]{
            #include "seals_train_c.csv"
        };

        double dataTest[nTest][nFeatures]{
            #include "seals_test_c.csv"
        };

        //  copy contents of the arrays over to better vector format
        for (int i=0; i < nFeatures; i++) {
            for (int j=0; j < nTrain; j++) {trainingSet[j][i] = dataTrain[j][i];}
            for (int j=0; j < nTest; j++) {testingSet[j][i] = dataTest[j][i];}
        }

        std::cout << "Finish initialization of data...\n";
    }

    // returns the root node of the decision tree
    Node buildTree() {

        // TODO finish this function
        Node testNode(0);

        buildTree(&trainingSet);

        return testNode;
    }

  private:

    // iteratively return optimized nodes
    Node buildTree(std::vector<std::vector<double>>* dataset) {

        double* addr = calculateBestSplit(dataset);

        // TODO finish this function

        Node testNode(1);
        return testNode;
    }

    double* calculateBestSplit(std::vector<std::vector<double>>* dataset) {

        double InfoGain = -1e10, CutFeature = 0, CutThreshold = 0;
        // std::vector<std::vector<double>> data = *dataset;
        std::vector<std::vector<double>> data = *dataset;

        for (int feature = 1; feature < nFeatures; feature++) {

            // prepare possible threshold values
            std::vector<double> thresholds;
            for (std::vector<double> datapoint : *dataset){thresholds.push_back(datapoint[feature]);}

            // calculate information gain for each threshold
            for (double threshold : thresholds) {
                std::vector<std::vector<double>> dataLeft, dataRight;

                // this causes a segmentation fault for some reason (=
                std::copy_if(data.begin(),data.end(), dataLeft.begin(),
                        [feature, threshold](std::vector<double> x) { return x[feature] != threshold;});

                // TODO: FIX THIS


            }


        }

        double cutPerformance[3] = {InfoGain,CutFeature,CutThreshold};
        double* ptr = cutPerformance;

        return ptr;
    }
    

    

};