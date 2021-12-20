#include <algorithm>
#include <iostream>
#include <chrono>
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
        double cut_threshold;                // value of the node cut threshold
        int cut_feature;                     // feature that is compared at node
        int leaf_class;                      // (if applicable) the leaf class
        double leaf_purity;                  // training purity for leaf node
        bool is_leaf;                        // whether node is leaf or not

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
            is_leaf = false;

        }

        Node(int cls, double purity) {

            // constructor for leaf nodes
            leaf_purity = purity;
            leaf_class = cls;
            is_leaf = true;

        }

        void setRoot(int feature, double threshold, double gain, Node* left, Node* right) {
            
            cut_feature = feature;
            cut_threshold = threshold;
            information_gain = gain;
            child_left = left;
            child_right = right;
            is_leaf = false;

        }

        void printNode() const {

            if (!is_leaf) {
                printf("Cut feature:   %i\n",cut_feature);
                printf("Cut threshold: %f\n",cut_threshold);
                printf("Cut info gain: %f\n",information_gain);
                printf("left child:    %p\n",child_left);
                printf("right child:   %p\n\n",child_right);
            }
            else {
                printf("Leaf class:    %i\n", leaf_class);
                printf("Leaf purity:   %.2f\%\n\n",leaf_purity);
            }


        }

};

class DecisionTree {

  public:

    // have to hardcode this at compile time the way it is now =(
    const size_t n_train = 1207, n_test = 213;
    const size_t n_features = 129;
    int max_tree_depth, minimum_samples;
    double training_performance = 0;
    int n_nodes = 1, n_leaves = 0;

    dmatrix training_set {n_train, drow (n_features) };
    dmatrix testing_set {n_test, drow (n_features) };

    // allocate nodes on the heap, since we need them outside of buildTree()
    // I dunno if this is the most efficient way, it works for now *shrug*
    Node* root_node = new Node(0, 0, 0, nullptr, nullptr);

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
    void buildTree(bool verbose=false) {

        auto start = std::chrono::high_resolution_clock::now();

        dcut root_split = calculateBestSplit(training_set, verbose);

        size_t feature = std::get<0>(root_split);
        double threshold = std::get<1>(root_split);
        double info_gain = std::get<2>(root_split);

        auto data_split = splitDataset(training_set, feature, threshold);
        dmatrix data_left = data_split[0], data_right = data_split[1];

        Node* node_left = buildTree(data_left, 1, verbose);
        Node* node_right = buildTree(data_right, 1, verbose);

        root_node->setRoot(feature, threshold, info_gain, node_left, node_right);

        auto stop = std::chrono::high_resolution_clock::now();
        auto training_time = std::chrono::duration_cast<std::chrono::seconds>(stop-start);

        std::cout << "\nFinished building decision tree...\n";
        std::cout << "maximum tree depth:   " << max_tree_depth << "\n";
        std::cout << "minimum sample size:  " << minimum_samples << "\n\n";
        std::cout << "n_nodes / n_leaves:   " << n_nodes << "/" << n_leaves << "\n";
        std::cout << "(theoretical max):    (" << pow(2,max_tree_depth)-1 << "/" << pow(2,max_tree_depth-1) << ")\n\n";
        std::cout << "Training performance: " << training_performance << "%\n";
        std::cout << "Training time:        " << training_time.count() << "s\n\n";
    }

    // predict testing set and measure performance
    void predictTest() {

        // TODO write this

    }

  private:

    // iteratively return optimized nodes
    Node* buildTree(dmatrix& dataset, size_t depth, bool verbose = false) {

        // keep returning (non-leaf) nodes if stopping conditions aren't met
        if (depth + 1 < max_tree_depth && dataset.size() > minimum_samples) {

            dcut root_split = calculateBestSplit(dataset, verbose);

            size_t feature = std::get<0>(root_split);
            double threshold = std::get<1>(root_split);
            double info_gain = std::get<2>(root_split);

            auto data_split = splitDataset(dataset, feature, threshold);
            dmatrix data_left = data_split[0], data_right = data_split[1];

            Node* node_left = buildTree(data_left, depth + 1, verbose);
            Node* node_right = buildTree(data_right, depth + 1, verbose);

            Node* this_node = new Node(feature, threshold, info_gain, node_left, node_right);

            n_nodes +=1;

            return this_node;
        }
        // else return leaf node based on remaining dataset
        else {

            dcolumn labels = extractColumn(dataset, 0);

            int leaf_class;
            unsigned long ones = std::count(labels.begin(),labels.end(),1);

            if (ones > dataset.size() - ones) {leaf_class = 1;}
            else {leaf_class = 0;}

            double purity = std::max(ones, dataset.size()-ones)/(double)dataset.size() * 100;
            training_performance += purity * dataset.size()/training_set.size();

            Node* leaf_node = new Node(leaf_class, purity);

            n_nodes +=1, n_leaves +=1;

            return leaf_node;
        }
    }

    // return feature, threshold that maximize information gain
    dcut calculateBestSplit(dmatrix& dataset, bool verbose=false) {

        size_t best_feature = 0;
        double best_threshold = 0;
        double best_information = -1e10;
        double purity_left, purity_right;

        dcolumn labels = extractColumn(dataset, 0);
        dcolumn labels_left, labels_right;

        double len_parent = dataset.size();
        size_t parent_zeros = std::count(labels.begin(),labels.end(),0);
        size_t parent_ones = std::count(labels.begin(),labels.end(),1);

        // iterate over all features (i=0 is the label!)
        for (size_t i = 1; i < n_features; i++) {

            if (verbose) {std::cout << "\rCalculating best split " << i << "/" << n_features-1;}
            
            // all possible threshold values
            auto features = extractColumn(dataset, i);

            // iterate over all threshold values
            for (size_t j = 0; j < dataset.size(); j++) {

                for (const auto &row : dataset) {
                    // split data into left and right child based on threshold
                    if (row[i] <= features[j]) {labels_left.push_back(row[0]);}          // value is smaller than threshold, go left
                    else if (row[i] > features[j]) {labels_right.push_back(row[0]);}     // value is larger than threshold, go right
                }

                double len_left = labels_left.size();
                double len_right = labels_right.size();

                // ensure children aren't empty
                if (len_left==0 || len_right==0) {continue;}

                // evaluate information gain from cutting at threshold
                size_t left_zeros = std::count(labels_left.begin(),labels_left.end(),0);
                size_t left_ones = std::count(labels_left.begin(),labels_left.end(),1);
                size_t right_zeros = std::count(labels_right.begin(),labels_right.end(),0);
                size_t right_ones = std::count(labels_right.begin(),labels_right.end(),1);

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
    std::vector<dmatrix> splitDataset(dmatrix& dataset, size_t i, double x) const {
         
        dmatrix child_left, child_right;

        std::copy_if(dataset.begin(),dataset.end(),std::back_inserter(child_left),[i,x](drow M){return M[i] <= x;});
        std::copy_if(dataset.begin(),dataset.end(),std::back_inserter(child_right),[i,x](drow M){return M[i] > x;});

        std::vector<dmatrix> split_result = {child_left,child_right};

        return split_result;      
    }

    // extract column i ( (i-1)th feature) from 2D dataset
    dcolumn extractColumn(dmatrix& dataset, size_t i) const {

        drow row_vector;

        for (const auto &row : dataset) {row_vector.push_back(row[i]);}

        return row_vector;
    }

};

int main() {

    DecisionTree PupClassifier(10,10);
    PupClassifier.buildTree();

    return 0;
}
