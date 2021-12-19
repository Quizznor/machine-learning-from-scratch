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
    Node buildTree() {

        // Workflow here: calculate best split across whole dataset once
        // and create root node with children stemming from recursion

        dcut best_split = calculateBestSplit(&training_set);

        int feature = std::get<0>(best_split);
        double threshold = std::get<1>(best_split);
        double info_gain = std::get<2>(best_split);

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

    // something's not quite yet working with this code, but it's faster!
    dcut calculateBestSplit(dmatrix* dataset) {

        int best_feature = 0;
        double best_threshold = 0;
        double best_information = -1e10;
        double purity_left, purity_right;

        dcolumn labels = extractColumn(dataset, 0);
        dcolumn labels_left, labels_right;

        int len_parent = dataset->size();
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
                    if (row[i] <= features[j]) {labels_left.push_back(labels[i]);}          // value is smaller than threshold, go left
                    else if (row[i] > features[j]) {labels_right.push_back(labels[i]);}     // value is larger than threshold, go right
                }

                int len_left = labels_left.size();
                int len_right = labels_right.size();

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

        auto best_split = splitDataset(dataset,best_feature, best_threshold);
        dmatrix child_left = best_split[0], child_right = best_split[1];

        dcut best_cut = std::make_tuple(best_feature, best_threshold, best_information);

        printf("\nBest split found @ (feature, threshold, info) = (%i, %f, %f)\n",best_feature, best_threshold, best_information);
        printf("Remaining dataset purities: %.2f\% (left) %.2f\% (right)\n", purity_left, purity_right);

        printMatrix(child_left);
        printMatrix(child_right);

        return best_cut;
    }

    // dcut calculateBestSplit(dmatrix* dataset) {

    //     // Workflow here: Iterate over all features and thresholds
    //     // calculate information gain for each one and return the 
    //     // one that yields the highest information gain

    //     // initial placeholder values
    //     int best_feature = 0;
    //     double current_info;
    //     double best_info = -1e10;
    //     double best_threshold = 0;
    //     double weight_left, weight_right;
    //     int left_ones, left_zeros, score_left;
    //     int right_ones, right_zeros, score_right;
    //     int parent_ones, parent_zeros, score_parent;

    //     dmatrix data_left, data_right;
    //     dmatrix best_data_left, best_data_right;
    //     dcolumn thresholds, thresholds_reduced;
    //     std::vector<dmatrix> split_by_feature;
    //     std::vector<dmatrix> left_sorted_by_class;
    //     std::vector<dmatrix> right_sorted_by_class;
    //     std::vector<dmatrix> parent_sorted_by_class;

    //     // iterate over all features (i=0 is the label!)
    //     for (int i = 1; i < n_features; i++) {

    //         std::cout << "\rCalculating best split " << i << "/" << n_features-1;
            
    //         // extract ~10% of the dataset to generate possible thresholds (and delete highest/lowest value)
    //         for (auto row = dataset->begin(); row < dataset->end(); row+=10) {thresholds.push_back((*row)[i]);}
    //         // ^^ maybe something linspace-esque is quicker (but less accurate)?
    //         // or just extract two columns? The only ones that really matter

    //         std::sort(thresholds.begin(),thresholds.end(),std::greater<double>{});
    //         thresholds_reduced = std::vector(&thresholds[1],&thresholds[thresholds.size()-1]);
            
    //         // iterate over all feature values
    //         for (const auto &x : thresholds_reduced) {
    //             split_by_feature = splitDataset(dataset, i, x);

    //             // calculate information gain and update best cut parameters accordingly
    //             data_left = split_by_feature[0], data_right = split_by_feature[1];
    //             left_sorted_by_class = splitDataset(&data_left,0,0.5);
    //             right_sorted_by_class = splitDataset(&data_right,0,0.5);
    //             parent_sorted_by_class = splitDataset(dataset,0,0.5);

    //             weight_left = data_left.size()/dataset->size(), weight_right = data_right.size()/dataset->size();

    //             left_ones = left_sorted_by_class[1].size(), left_zeros = left_sorted_by_class[0].size();
    //             right_ones = right_sorted_by_class[1].size(), right_zeros = right_sorted_by_class[0].size();
    //             parent_ones = parent_sorted_by_class[1].size(), parent_zeros = parent_sorted_by_class[0].size();

    //             score_left = 1 - (pow(left_ones/data_left.size(),2) + pow(left_zeros/data_left.size(),2));
    //             score_right = 1 - (pow(right_ones/data_right.size(),2) + pow(left_zeros/data_right.size(),2));
    //             score_parent = pow(left_ones/data_left.size(),2) + pow(left_zeros/data_left.size(),2);

    //             current_info = score_parent - ( weight_left * score_left + weight_right - score_right );

    //             if (current_info > best_info) {
    //                 best_data_right = data_right;
    //                 best_data_left = data_left;
    //                 best_info = current_info;
    //                 best_threshold = x;
    //                 best_feature = i;
    //             }

    //             // clean up after each threshold
    //             data_right.clear(), data_left.clear();
    //             split_by_feature.clear();
    //         }

    //         // clean up after each feature
    //         thresholds_reduced.clear(), thresholds.clear();
    //     }

    //     dcut best_cut = std::make_tuple(best_feature, best_threshold, best_info, best_data_left, best_data_right);

    //     printf("\nBest split found @ (feature, threshold, info) = (%i, %f, %f)\n",best_feature, best_threshold, best_info);

    //     printMatrix(best_data_left);
    //     printMatrix(best_data_right);

    //     return best_cut;
    // }

    // split a 2D dataset into 2 subchunks based on a feature i and cut threshold x
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
