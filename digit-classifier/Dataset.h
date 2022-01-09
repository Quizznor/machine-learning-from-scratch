class Dataset 
{
    public:

        uint32_t n_images;

        std::vector<Vector<uint8_t>> images;
        std::vector<uint8_t> labels;

        // parse image data from binary file
        void read_MNIST(std::string file_name, int n_max=0) 
        {
            std::ifstream file (file_name);

            std::cout << "Reading " << file_name << "... ";
            
            // read header information
            if (file.is_open()) 
            {
                file.read ((char*)&magic_number, sizeof(magic_number));
                file.read ((char*)&n_images, sizeof(n_images));
            }

            else {std::cout << "Something went wrong when reading " << file_name << std::endl;}

            // because data format is big endian (vs. me using small endian)
            magic_number = __builtin_bswap32(magic_number);
            n_images = __builtin_bswap32(n_images);

            // only extract n_max images/labels from file
            n_images = (n_max==0) ? n_images : n_max;

            // 2051 = magic number for image file
            if (magic_number==2051) 
            {
                file.read ((char*)&size_x, sizeof(size_x));
                file.read ((char*)&size_y, sizeof(size_y));

                size_x = __builtin_bswap32(size_x);
                size_y = __builtin_bswap32(size_y);

                // read n_max images
                for (int counter = 0; counter < n_images; counter++) 
                {
                    Vector<uint8_t> Image(image_height * image_width);

                    for (auto &pixel : Image.vector) 
                    {
                        file.read ((char*)&pixel,1);
                        pixel = unsigned(pixel);
                    }
                    
                    images.push_back(Image);
                }
            }
            // 2049 = magic number for label file
            else if(magic_number==2049) 
            {
                for (int i = 0; i < n_images; i++) 
                {
                    uint8_t label;
                    file.read ((char*)&label, sizeof(label));
                    labels.push_back(unsigned(label));
                }
            }

            file.close();

            std::cout << "done!" << std::endl;
        }

        // split dataset into n_batch shuffled subsets
        std::vector<std::vector<int>> create_batches(int n_batches)
        {
            int target_size = (images.size() - 1)/ n_batches + 1;

            std::vector<int> indices(images.size(),0);
            std::iota(indices.begin(),indices.end(),0);
            std::random_shuffle(indices.begin(),indices.end(), RNG);

            std::vector<std::vector<int>> result(n_batches,std::vector<int>());

            for (int i = 0; i < n_batches; ++i)
            {
                auto start_itr = std::next(indices.cbegin(), i*target_size);
                auto end_itr = std::next(indices.cbegin(), i*target_size + target_size);

                result[i].resize(target_size);

                if (i*target_size + target_size > indices.size())
                {
                    end_itr = indices.cend();
                    result[i].resize(indices.size() - i*target_size);
                }

                std::copy(start_itr, end_itr, result[i].begin());
            }

        return result;
        } 

        // print (formatted) content of image to std::cout
        void print_image(int index)
        {
            auto this_image = images[index].vector;

            for (int i = image_width; i < image_height * image_width; i++)
            {
                std::cout << std::setw(2) << unsigned(this_image[i]) / 10;
                ((i+1)%image_width)?std::cout<<" ":std::cout << std::endl;
            }
        }

    private:

        uint32_t magic_number;
        uint32_t size_x, size_y;

        static int RNG (int i) { return std::rand()%i;}
        
};