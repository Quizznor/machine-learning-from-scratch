#include <algorithm>
#include <iostream>
#include <vector>
#include <limits>
#include <cmath>
#include <tuple>

typedef std::vector<double> drow, dcolumn;
typedef std::vector<std::vector<double>> dmatrix;
typedef std::tuple<int,double,double> dcut;

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
    const unsigned long n_train = 1207, n_test = 213;
    const unsigned long n_features = 129;
    int max_tree_depth, minimum_samples;
    Node root_node;

    dmatrix training_set {n_train, drow (n_features) };
    dmatrix testing_set {n_test, drow (n_features) };

    DecisionTree(int depth, int samples) {

        max_tree_depth = depth;
        minimum_samples = samples;

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
    void buildTree() {

        dcut root_split = calculateBestSplit(&training_set);

        int feature = std::get<0>(root_split);
        double threshold = std::get<1>(root_split);
        double info_gain = std::get<2>(root_split);

        // init root node and all subsequent nodes recursively
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

    // return feature, threshold that maximize information gain
    dcut calculateBestSplit(dmatrix* dataset, bool verbose=false) {

        int best_feature = 0;
        double best_threshold = 0;
        double best_information = -1e10;
        double purity_left, purity_right;

        dcolumn labels = extractColumn(dataset, 0);
        dcolumn labels_left, labels_right;

        double len_parent = dataset->size();
        int parent_zeros = std::count(labels.begin(),labels.end(),0);
        int parent_ones = std::count(labels.begin(),labels.end(),1);

        for (int i = 1; i < n_features; i++) {

            std::cout << "\rCalculating best split " << i << "/" << n_features-1;

            // all possible threshold values
            auto features = extractColumn(dataset, i);

            // iterate over all threshold values
            for (int j = 0; j < dataset->size(); j++) {
                for (const auto &row : *dataset) {

                    // split data into left and right child based on threshold
                    if (row[i] <= features[j]) {labels_left.push_back(row[0]);}          // value is smaller than threshold, go left
                    else if (row[i] > features[j]) {labels_right.push_back(row[0]);}     // value is larger than threshold, go right
                }

                double len_left = labels_left.size();
                double len_right = labels_right.size();

                // ensure children aren't empty
                if (len_left==0 || len_right==0) {continue;}

                // evaluate information gain from cutting at threshold
                int left_zeros = std::count(labels_left.begin(),labels_left.end(),0);
                int left_ones = std::count(labels_left.begin(),labels_left.end(),1);
                int right_zeros = std::count(labels_right.begin(),labels_right.end(),0);
                int right_ones = std::count(labels_right.begin(),labels_right.end(),1);

                double score_left = 1 - (pow(left_ones/len_left,2) + pow(left_zeros/len_left,2));
                double score_right = 1 - (pow(right_ones/len_right,2) + pow(right_zeros/len_right,2));
                double score_parent = 1 - (pow(parent_ones/len_parent,2) + pow(parent_zeros/len_parent,2));

                double current_info = score_parent - ( len_left/len_parent * score_left + len_right/len_parent * score_right );

                // update best cut parameters if information gain is larger than previous
                if (current_info > best_information) {
                    best_information = current_info;
                    best_threshold = features[j];
                    best_feature = i;

                    // calculate purity for fun
                    purity_left = std::max(left_zeros,left_ones)/len_left * 100;
                    purity_right = std::max(right_zeros,right_ones)/len_right * 100;
                }               

                labels_left.clear(), labels_right.clear();
            }
        }

        dcut best_cut = std::make_tuple(best_feature, best_threshold, best_information);

        if (verbose) {
            printf("\nBest split found @ (x[i], x[i] <= ?, info) = (%i, %f, %f)\n",best_feature, best_threshold, best_information);
            printf("Remaining dataset purities: %.2f\% (left) %.2f\% (right)\n", purity_left, purity_right);

            auto best_split = splitDataset(dataset,best_feature, best_threshold);
            dmatrix child_left = best_split[0], child_right = best_split[1];

            printMatrix(child_left);
            printMatrix(child_right);
        }

        return best_cut;
    }

    // split dataset into two subchunks with x[i] <= x and x[i] > x respectively
    std::vector<dmatrix> splitDataset(dmatrix* dataset, int i, double x) {
         
        dmatrix child_left, child_right;

        std::copy_if(dataset->begin(),dataset->end(),std::back_inserter(child_left),[i,x](drow M){return M[i] <= x;});
        std::copy_if(dataset->begin(),dataset->end(),std::back_inserter(child_right),[i,x](drow M){return M[i] > x;});

        std::vector<dmatrix> split_result = {child_left,child_right};

        return split_result;      
    }

    // extract column i ( (i-1)th feature) from 2D dataset
    dcolumn extractColumn(dmatrix* dataset, int i) {

        drow row_vector;

        for (const auto &row : *dataset) {row_vector.push_back(row[i]);}

        return row_vector;
    }


};

int main() {

  DecisionTree PupClassifier(3,10);
  PupClassifier.buildTree();

  return 0;
}
