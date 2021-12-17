#include <algorithm>
#include <iostream>
#include <vector>
#include <limits>
#include <cmath>
#include <tuple>

typedef std::vector<double> drow, dcolumn;
typedef std::vector<std::vector<double>> dmatrix;
typedef std::tuple<int,double,double,dmatrix,dmatrix> dcut;

// for debugging purposes
void printMatrix(dmatrix dataset) {

    int n_rows = dataset.size(), n_cols = dataset[0].size();

    std::cout << "\nMATRIX SHAPE: (" << n_rows << "," << n_cols << ")" << std::endl;
    
    for (int i = 0; i < n_rows; i++) {
        if (i==0 || i==1 || i==n_cols-2 || i==n_cols-1) {
            for (int j = 0; j < n_cols; j++) {
                if (j==0 || j==1 || j==n_rows-1) {
                    std::cout  << dataset[i][j] << " ";}
                else if (j==2) {std::cout << "... ";}
                else if (j==n_cols-1) {std::cout  << dataset[i][j] << "\n";}
            }
        }
        else if (i==2) {std::cout << "  ...\n";}
    }

    std::cout << "\n";
}

class Node {

    public:

        // node specifiers
        double information_gain;             // information gained from node cut
        int cut_threshold;                   // value of the node cut threshold
        int cut_feature;                     // feature that is compared at node
        int leaf_class;                      // (if applicable) the leaf class

        // set up tree links
        Node* child_left;                    // link for data <= threshold 
        Node* child_right;                   // link for data >  threshold
        
        Node(int feature, double threshold, double gain, Node* left, Node* right) {
            
            // constructor for non-leaf nodes
            cut_feature = feature;
            cut_threshold = threshold;
            information_gain = gain;
            child_left = left;
            child_right = right;

        }

        Node(int cls, int gain) {

            // constructor for leaf nodes
            information_gain = gain;
            leaf_class = cls;

        }
};

class DecisionTree {

  public:

    // have to hardcode this at compile time the way it is now =(
    const unsigned long n_train = 1207, n_test = 213, n_features = 129;

    dmatrix training_set {n_train, drow (n_features) };
    dmatrix testing_set {n_test, drow (n_features) };

    DecisionTree(int depth) {

        int maxTreeDepth = depth;

        double data_train[n_train][n_features]{
            #include "seals_train_c.csv"
        };

        double data_test[n_test][n_features]{
            #include "seals_test_c.csv"
        };

        //  copy contents of the arrays over to better vector format
        for (int i=0; i < n_features; i++) {
            for (int j=0; j < n_train; j++) {training_set[j][i] = data_train[j][i];}
            for (int j=0; j < n_test; j++) {testing_set[j][i] = data_test[j][i];}
        }

        std::cout << "Finish initialization of data...\n";
    }

    // returns the root node of the decision tree
    Node buildTree() {

        // Workflow here: calculate best split across whole dataset once
        // and create root node with children stemming from recursion

        dcut best_split = calculateBestSplit(&training_set);

        int feature = std::get<0>(best_split);
        double threshold = std::get<1>(best_split);
        double info_gain = std::get<2>(best_split);
        dmatrix child_left = std::get<3>(best_split);
        dmatrix child_right = std::get<4>(best_split);

        printMatrix(child_left);
        printMatrix(child_right);

        Node TestNode(0,0);
        return TestNode;
    }

  private:

    // iteratively return optimized nodes
    Node buildTree(dmatrix* dataset) {

        // Workflow here: Check if stopping conditions are met, if not,
        // calculate best split, raise node with call to itself to create
        // children. If stopping conditions raise leaf node

        // if (/* stopping condition NOT met*/) {
        //     // keep creating nodes
        // }
        // else {
        //     // return leaf node based on dataset
        // }

    
        Node TestNode(1,0);
        return TestNode;
    }

    dcut calculateBestSplit(dmatrix* dataset) {

        // Workflow here: Iterate over all features and thresholds
        // calculate information gain for each one and return the 
        // one that yields the highest information gain

        // initial placeholder values
        int best_feature = 0;
        double best_threshold = 0;
        double best_info = -1e10;
        dmatrix data_left, data_right;
        dcolumn thresholds, thresholds_reduced;
        std::vector<dmatrix> split;

        for (int i = 0; i < n_features; i++) {

            std::cout << "\rCalculating best split " << i << "/" << n_features-1;

            // get all elements in feature column (minus 10 lowest and highest value)
            for (const auto &row : *dataset) {thresholds.push_back(row[i]);}

            std::sort(thresholds.begin(),thresholds.end(),std::greater<double>{});
            thresholds_reduced = std::vector(&thresholds[11],&thresholds[thresholds.size()-11]);

            // iterate over all feature values
            for (const auto &threshold : thresholds_reduced) {
                split = splitDataset(dataset, i, threshold);

                // calculate information gain and update best cut parameters accordingly

                split.clear();
            }

            thresholds_reduced.clear(), thresholds.clear();
        }

        // some more placeholder values
        data_left = {{1,2,3},{4,5,6}};
        data_right = {{1,2,3},{4,5,6}};

        dcut best_cut = std::make_tuple(best_feature, best_threshold, best_info, data_left, data_right);

        return best_cut;
    }

    // split a 2D dataset into 2 subchunks based on a feature i and cut threshold x
    std::vector<dmatrix> splitDataset(dmatrix* dataset, int i, double x) {
         
        dmatrix child_left, child_right;

        std::copy_if(dataset->begin(),dataset->end(),std::back_inserter(child_left),[i,x](drow M){return M[i] <= x;});
        std::copy_if(dataset->begin(),dataset->end(),std::back_inserter(child_right),[i,x](drow M){return M[i] > x;});

        std::vector<dmatrix> split_result = {child_left,child_right};

        return split_result;      
    }

};

int main() {

  DecisionTree PupClassifier(3);
  PupClassifier.buildTree();

  return 0;
}
