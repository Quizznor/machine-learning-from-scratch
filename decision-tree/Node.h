class Node {

  public:
    Node(int feature, double threshold, double gain, Node left, Node right) {

        // constructor for non-leaf nodes
        int cutFeature = feature;         // feature # that is compared
        double cutThreshold = threshold;  // cut threshold for feature
        double informationGain = gain;    // information gain from cut
    
    }

    Node(int cls) {

        // constructor for leaf nodes
        int leafClass = cls;              // class value of leaf node
    }
};